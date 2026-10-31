//! Registration and bounded delivery of macOS hotkey callbacks.
//!
//! The native tap only looks up registration identities and tries to enqueue
//! them. A separate worker invokes callbacks in delivery order. Neither native
//! events nor callback-owned data cross the native callback's lifetime boundary.
//! Callers close delivery before stopping the tap; an already selected callback
//! may finish, but queued callbacks are canceled on close or unregistration.

use crate::{Error, Hotkey, Result, macos};
use std::{
    collections::{HashMap, hash_map::Entry},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
        mpsc::{self, SyncSender, TrySendError},
    },
    thread,
};

// Bound both retained work and stale input after a callback stalls. Sixty-four
// pending presses permit short bursts without building an unlimited action
// backlog. When full, the native tap forwards the key instead of consuming an
// action it cannot deliver. try_send never waits for the callback worker.
const QUEUE_CAPACITY: usize = 64;

type Callback = Arc<Mutex<Box<dyn FnMut() + Send>>>;

struct Registration {
    // A queued identity cannot be reused, even if the same key is unregistered
    // and registered again. Its final drop never runs arbitrary callback data.
    identity: Arc<()>,
    callback: Callback,
}

struct Pending {
    hotkey: Hotkey,
    identity: Arc<()>,
}

#[derive(Default)]
struct Registry {
    hotkeys: Mutex<HashMap<Hotkey, Registration>>,
    closed: AtomicBool,
}

struct Worker(Arc<Registry>);

impl Drop for Worker {
    fn drop(&mut self) {
        // Also runs during callback panic; no subsequent input is consumed.
        self.0.closed.store(true, Ordering::Release);
    }
}

pub struct Dispatcher {
    registry: Arc<Registry>,
    sender: SyncSender<Pending>,
}

impl Dispatcher {
    pub fn new() -> Result<Self> {
        let registry = Arc::new(Registry::default());
        let worker = Worker(registry.clone());
        let (sender, receiver) = mpsc::sync_channel::<Pending>(QUEUE_CAPACITY);

        // The worker owns no sender, so releasing the tap and Hook disconnects
        // the receiver. Do not join here: callbacks are arbitrary user code and
        // can drop their Hook or block indefinitely. No callback holds the
        // registry lock, including while its captured values are destroyed.
        thread::Builder::new()
            .name("hotkey callbacks".into())
            .spawn(move || {
                let worker_registry = &worker.0;
                for pending in receiver {
                    if worker_registry.closed.load(Ordering::Acquire) {
                        break;
                    }

                    let callback = {
                        let hotkeys = worker_registry
                            .hotkeys
                            .lock()
                            .expect("callback execution never holds the registration lock");
                        if worker_registry.closed.load(Ordering::Acquire) {
                            None
                        } else {
                            hotkeys.get(&pending.hotkey).and_then(|registration| {
                                Arc::ptr_eq(&registration.identity, &pending.identity)
                                    .then(|| registration.callback.clone())
                            })
                        }
                    };

                    if let Some(callback) = callback {
                        // Only this worker invokes callbacks. The separate mutex
                        // makes their Send-only storage shareable with registration
                        // owners; the tap never acquires it. Unregister/close may
                        // race with an invocation already selected here.
                        let mut callback = callback
                            .lock()
                            .expect("only the callback worker invokes this registration");
                        callback();
                    }
                }
            })
            .map_err(macos::Error::CouldntCreateCallbackThread)?;

        Ok(Self { registry, sender })
    }

    pub fn register<F>(&self, hotkey: Hotkey, callback: F) -> Result<()>
    where
        F: FnMut() + Send + 'static,
    {
        // Allocate and destroy user-owned captures outside the registry lock.
        let registration = Registration {
            identity: Arc::new(()),
            callback: Arc::new(Mutex::new(Box::new(callback))),
        };
        let mut hotkeys = self
            .registry
            .hotkeys
            .lock()
            .expect("callback execution never holds the registration lock");

        if self.registry.closed.load(Ordering::Acquire) {
            return Err(macos::Error::ThreadStoppedUnexpectedly.into());
        }

        if let Entry::Vacant(entry) = hotkeys.entry(hotkey) {
            entry.insert(registration);
            Ok(())
        } else {
            Err(Error::AlreadyRegistered)
        }
    }

    pub fn unregister(&self, hotkey: Hotkey) -> Result<()> {
        let registration = self
            .registry
            .hotkeys
            .lock()
            .expect("callback execution never holds the registration lock")
            .remove(&hotkey)
            .ok_or(Error::NotRegistered)?;
        drop(registration);
        Ok(())
    }

    /// Returns whether the action was admitted, which is also the native tap's
    /// consume decision. No callback or callback destructor runs on this path.
    /// The fixed-capacity channel is allocated at construction; each matched
    /// press only clones a small identity handle and attempts an enqueue.
    pub fn enqueue(&self, hotkey: Hotkey) -> bool {
        if self.registry.closed.load(Ordering::Acquire) {
            return false;
        }

        let identity = self
            .registry
            .hotkeys
            .lock()
            .expect("callback execution never holds the registration lock")
            .get(&hotkey)
            .map(|registration| registration.identity.clone());
        let Some(identity) = identity else {
            return false;
        };

        match self.sender.try_send(Pending { hotkey, identity }) {
            Ok(()) => true,
            // Full is deliberate backpressure; disconnected means teardown or
            // a panicked callback worker. In both cases preserve native input.
            Err(TrySendError::Full(_) | TrySendError::Disconnected(_)) => false,
        }
    }

    pub fn close(&self) {
        // Order shutdown against the worker's selection, just like unregister.
        // The guard protects only the decision, never callback execution/drop.
        let _hotkeys = self
            .registry
            .hotkeys
            .lock()
            .expect("callback execution never holds the registration lock");
        self.registry.closed.store(true, Ordering::Release);
    }
}

impl Drop for Dispatcher {
    fn drop(&mut self) {
        self.close();
    }
}

#[cfg(test)]
mod tests {
    use super::{Dispatcher, QUEUE_CAPACITY};
    use crate::{Error, KeyCode, Modifiers, macos};
    use std::{
        sync::{
            Arc, Mutex, Weak,
            atomic::{AtomicUsize, Ordering},
            mpsc,
        },
        thread,
        time::{Duration, Instant},
    };

    // Failure deadline for worker rendezvous; production never waits on these.
    const DEADLINE: Duration = Duration::from_secs(5);

    struct ReenterOnDrop {
        dispatch: Weak<Dispatcher>,
        completed: mpsc::Sender<()>,
    }

    impl Drop for ReenterOnDrop {
        #[track_caller]
        fn drop(&mut self) {
            let dispatch = self.dispatch.upgrade().unwrap();
            let key = KeyCode::KeyB.with_modifiers(Modifiers::empty());
            dispatch.register(key, || {}).unwrap();
            assert!(dispatch.enqueue(key));
            self.completed.send(()).unwrap();
        }
    }

    #[test]
    fn unregister_destroys_captures_outside_the_registration_lock() {
        let dispatch = Arc::new(Dispatcher::new().unwrap());
        let (completed, completions) = mpsc::channel();
        let capture = ReenterOnDrop {
            dispatch: Arc::downgrade(&dispatch),
            completed,
        };
        let key = KeyCode::KeyA.with_modifiers(Modifiers::empty());
        dispatch
            .register(key, move || {
                let _ = &capture;
            })
            .unwrap();
        let worker_dispatch = dispatch.clone();
        thread::spawn(move || worker_dispatch.unregister(key).unwrap());
        completions.recv_timeout(DEADLINE).unwrap();
    }

    #[test]
    fn callback_can_replace_itself_and_drop_the_last_dispatch_owner() {
        let dispatch = Arc::new(Dispatcher::new().unwrap());
        let owner = Arc::new(Mutex::new(Some(dispatch.clone())));
        let callback_owner = owner.clone();
        let key = KeyCode::KeyA.with_modifiers(Modifiers::empty());
        let (started, starts) = mpsc::channel();
        let (release, releases) = mpsc::channel();
        let (finished, finishes) = mpsc::channel();
        let calls = Arc::new(AtomicUsize::new(0));
        let new_calls = calls.clone();
        dispatch
            .register(key, move || {
                started.send(()).unwrap();
                releases.recv_timeout(DEADLINE).unwrap();
                let dispatch = callback_owner.lock().unwrap().take().unwrap();
                dispatch.unregister(key).unwrap();
                let calls = new_calls.clone();
                dispatch
                    .register(key, move || {
                        calls.fetch_add(1, Ordering::Relaxed);
                    })
                    .unwrap();
                assert!(dispatch.enqueue(key));
                drop(dispatch);
                finished.send(()).unwrap();
            })
            .unwrap();
        assert!(dispatch.enqueue(key));
        starts.recv_timeout(DEADLINE).unwrap();
        assert!(dispatch.enqueue(key));
        let registry = Arc::downgrade(&dispatch.registry);
        drop(dispatch);
        release.send(()).unwrap();
        finishes.recv_timeout(DEADLINE).unwrap();
        let deadline = Instant::now() + DEADLINE;
        while registry.strong_count() != 0 {
            assert!(
                Instant::now() < deadline,
                "callback worker must exit after owner drop"
            );
            thread::yield_now();
        }
        assert!(owner.lock().unwrap().is_none());
        assert_eq!(calls.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn panicked_callback_closes_delivery_and_preserves_native_input() {
        let dispatch = Dispatcher::new().unwrap();
        let key = KeyCode::KeyA.with_modifiers(Modifiers::empty());
        dispatch
            .register(key, || panic!("intentional callback failure"))
            .unwrap();
        assert!(dispatch.enqueue(key));
        let deadline = Instant::now() + DEADLINE;
        while !dispatch.registry.closed.load(Ordering::Acquire) {
            assert!(
                Instant::now() < deadline,
                "callback panic must close delivery"
            );
            thread::yield_now();
        }
        assert!(!dispatch.enqueue(key));
        assert!(matches!(
            dispatch.register(key, || {}),
            Err(Error::Platform(macos::Error::ThreadStoppedUnexpectedly))
        ));
    }

    #[test]
    fn blocked_callback_preserves_admission_capacity_and_registration_identity() {
        let dispatch = Dispatcher::new().unwrap();
        let a = KeyCode::KeyA.with_modifiers(Modifiers::empty());
        let b = KeyCode::KeyB.with_modifiers(Modifiers::empty());
        let (started, starts) = mpsc::channel();
        let (release, releases) = mpsc::channel();
        let old_calls = Arc::new(AtomicUsize::new(0));
        let calls = old_calls.clone();
        dispatch
            .register(a, move || {
                calls.fetch_add(1, Ordering::Relaxed);
                started.send(()).unwrap();
                releases.recv_timeout(DEADLINE).unwrap();
            })
            .unwrap();
        assert!(dispatch.enqueue(a));
        starts.recv_timeout(DEADLINE).unwrap();

        assert!(matches!(
            dispatch.register(a, || {}),
            Err(Error::AlreadyRegistered)
        ));
        assert!(!dispatch.enqueue(b));
        let (barrier, barriers) = mpsc::channel();
        dispatch
            .register(b, move || barrier.send(()).unwrap())
            .unwrap();
        for _ in 0..QUEUE_CAPACITY - 1 {
            assert!(dispatch.enqueue(a));
        }
        assert!(dispatch.enqueue(b));
        assert!(
            !dispatch.enqueue(a),
            "a full queue must forward native input"
        );

        // Re-registration cannot redirect queued presses to the new callback.
        // These operations must complete while the old callback is blocked.
        dispatch.unregister(a).unwrap();
        assert!(matches!(dispatch.unregister(a), Err(Error::NotRegistered)));
        let (new_call, new_calls) = mpsc::channel();
        dispatch
            .register(a, move || new_call.send(()).unwrap())
            .unwrap();
        release.send(()).unwrap();
        barriers.recv_timeout(DEADLINE).unwrap();
        assert_eq!(old_calls.load(Ordering::Relaxed), 1);
        assert!(matches!(
            new_calls.try_recv(),
            Err(mpsc::TryRecvError::Empty)
        ));
        assert!(dispatch.enqueue(a));
        new_calls.recv_timeout(DEADLINE).unwrap();
    }

    #[test]
    fn close_cancels_pending_callbacks_without_waiting_for_running_callback() {
        struct Dropped(mpsc::Sender<()>);
        impl Drop for Dropped {
            #[track_caller]
            fn drop(&mut self) {
                self.0.send(()).unwrap();
            }
        }

        let dispatch = Dispatcher::new().unwrap();
        let a = KeyCode::KeyA.with_modifiers(Modifiers::empty());
        let b = KeyCode::KeyB.with_modifiers(Modifiers::empty());
        let (started, starts) = mpsc::channel();
        let (release, releases) = mpsc::channel();
        dispatch
            .register(a, move || {
                started.send(()).unwrap();
                releases.recv_timeout(DEADLINE).unwrap();
            })
            .unwrap();
        let (dropped, drops) = mpsc::channel();
        let guard = Dropped(dropped);
        let calls = Arc::new(AtomicUsize::new(0));
        let callback_calls = calls.clone();
        dispatch
            .register(b, move || {
                let _ = &guard;
                callback_calls.fetch_add(1, Ordering::Relaxed);
            })
            .unwrap();
        assert!(dispatch.enqueue(a));
        starts.recv_timeout(DEADLINE).unwrap();
        assert!(dispatch.enqueue(b));
        dispatch.close();
        assert!(!dispatch.enqueue(a));
        drop(dispatch);
        release.send(()).unwrap();
        drops.recv_timeout(DEADLINE).unwrap();
        assert_eq!(calls.load(Ordering::Relaxed), 0);
    }
}
