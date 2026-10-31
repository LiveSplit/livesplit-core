//! macOS callback regressions and an opt-in private-process event-tap test.
//!
//! Default tests create no taps or posted input. The ignored integration test
//! uses existing grants and a disposable windowless receiver so timeout stalls
//! and synthetic keys never target the foreground application. It does not
//! change TCC grants, validate revocation or simulate a user-requested disable.

use crate::{
    ConsumePreference, KeyCode, Modifiers,
    macos::{
        CallbackContext, Hook, Observation, Owned, RunLoop, State, Target, callback, cf,
        cg::{self, EventField, EventFlags, EventType},
        dispatch::Dispatcher,
        recovery::Recovery,
        shutdown::Shutdown,
    },
};
use std::{
    cell::Cell,
    env, fs,
    io::{BufRead, BufReader},
    path::PathBuf,
    process::{self, Child, Command, Stdio},
    ptr::null_mut,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicUsize, Ordering},
        mpsc::{self, Receiver},
    },
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[track_caller]
fn context(calls: Arc<AtomicUsize>) -> (CallbackContext, Receiver<()>) {
    let dispatch = Dispatcher::new().unwrap();
    let (delivered, deliveries) = mpsc::channel();
    dispatch
        .register(
            KeyCode::KeyA.with_modifiers(Modifiers::empty()),
            move || {
                calls.fetch_add(1, Ordering::Relaxed);
                delivered.send(()).unwrap();
            },
        )
        .unwrap();
    (
        CallbackContext {
            state: Arc::new(State {
                dispatch,
                stall_native: AtomicBool::new(false),
                observer: None,
            }),
            tap: Cell::new(null_mut()),
            recovery: Recovery::new(),
        },
        deliveries,
    )
}

#[track_caller]
fn dispatch_barrier(dispatch: &Dispatcher) {
    let key = KeyCode::KeyB.with_modifiers(Modifiers::empty());
    let (delivered, deliveries) = mpsc::channel();
    dispatch
        .register(key, move || delivered.send(()).unwrap())
        .unwrap();
    assert!(dispatch.enqueue(key));
    deliveries.recv_timeout(NATIVE_DEADLINE).unwrap();
    dispatch.unregister(key).unwrap();
}

#[test]
fn non_keyboard_messages_and_null_key_events_never_dispatch_hotkeys() {
    let calls = Arc::new(AtomicUsize::new(0));
    let (context, _deliveries) = context(calls.clone());
    let context_ptr = (&context as *const CallbackContext).cast_mut().cast();

    for event_type in [
        EventType::NULL,
        EventType::KEY_UP,
        EventType::FLAGS_CHANGED,
        EventType::KEY_DOWN,
    ] {
        // SAFETY: The context is live for every call. These messages carry
        // no usable key event and must return before touching native event
        // accessors or the unused tap. No native hook is constructed.
        let returned = unsafe { callback(null_mut(), event_type, null_mut(), context_ptr) };
        assert!(returned.is_null());
    }

    dispatch_barrier(&context.state.dispatch);
    assert_eq!(calls.load(Ordering::Relaxed), 0);
}

#[test]
fn native_key_dispatch_preserves_repeats_registration_and_consumption() {
    // Event construction stays local; no event is posted to the operating system.
    let calls = Arc::new(AtomicUsize::new(0));
    let (context, deliveries) = context(calls.clone());
    let context_ptr = (&context as *const CallbackContext).cast_mut().cast();

    for (key, handled) in [(ANSI_A, true), (ANSI_S, false)] {
        // SAFETY: A null source is supported. The returned event is owned
        // by this test and released by its CoreFoundation guard.
        let event = unsafe { cg::CGEventCreateKeyboardEvent(null_mut(), key, true) };
        assert!(!event.is_null(), "CoreGraphics must create the local event");
        let event = Owned(event);

        // SAFETY: This valid owned event is never shared or posted. Setting
        // all relevant fields makes dispatch independent of host modifiers.
        unsafe {
            cg::CGEventSetFlags(event.0, EventFlags::empty());
            cg::CGEventSetIntegerValueField(event.0, EventField::KEYBOARD_EVENT_AUTOREPEAT, 0);
            let result = callback(null_mut(), EventType::KEY_DOWN, event.0, context_ptr);
            assert_eq!(result.is_null(), handled);
            if !handled {
                assert_eq!(result, event.0);
            }

            cg::CGEventSetIntegerValueField(event.0, EventField::KEYBOARD_EVENT_AUTOREPEAT, 1);
            assert_eq!(
                callback(null_mut(), EventType::KEY_DOWN, event.0, context_ptr),
                event.0
            );
        }
    }

    deliveries.recv_timeout(NATIVE_DEADLINE).unwrap();
    dispatch_barrier(&context.state.dispatch);
    assert_eq!(calls.load(Ordering::Relaxed), 1);
}

// A bounded wait covers IPC, host scheduling and the deliberately slow callback.
// It is a failure deadline, not an assumption about CoreGraphics' timeout value.
const NATIVE_DEADLINE: Duration = Duration::from_secs(10);
// Stall only events addressed to the disposable receiver. Hosts whose native
// timeout exceeds this delay cannot validate recovery with this test.
pub const NATIVE_STALL: Duration = Duration::from_secs(2);
// Carbon virtual-key codes; S is the unregistered FIFO delivery barrier.
const ANSI_A: u16 = 0x00;
const ANSI_S: u16 = 0x01;

/// Own every compiler output, including during fixture setup failure. A unique
/// per-run directory avoids collisions after PID reuse and keeps Cargo's shared
/// deps directory free of files left by manual native test runs.
struct FixtureFiles {
    directory: PathBuf,
}

impl FixtureFiles {
    #[track_caller]
    fn new() -> Self {
        let directory = env::temp_dir().join(format!(
            "livesplit-recovery-{}-{}",
            process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        ));
        fs::create_dir(&directory).unwrap();
        Self { directory }
    }
}

impl Drop for FixtureFiles {
    fn drop(&mut self) {
        if let Err(error) = fs::remove_dir_all(&self.directory) {
            eprintln!(
                "could not remove native fixture {}: {error}",
                self.directory.display()
            );
        }
    }
}

struct NativeReceiver {
    child: Child,
    events: Receiver<String>,
    // NativeReceiver::drop reaps the child before field destructors remove files.
    files: FixtureFiles,
}

impl Drop for NativeReceiver {
    fn drop(&mut self) {
        // Closing our pipe lets the windowless fixture exit without a Cocoa
        // save/termination workflow. A kill also handles a stuck receiver.
        drop(self.child.stdin.take());
        match self.child.try_wait() {
            Ok(Some(_)) => return,
            Ok(None) => {
                if let Err(error) = self.child.kill() {
                    eprintln!("could not terminate native receiver: {error}");
                }
            }
            Err(error) => eprintln!("could not inspect native receiver: {error}"),
        }
        if let Err(error) = self.child.wait() {
            eprintln!("could not reap native receiver: {error}");
        }
    }
}

impl NativeReceiver {
    #[track_caller]
    fn new() -> Self {
        let files = FixtureFiles::new();
        eprintln!("native_receiver_directory={}", files.directory.display());
        let binary = files.directory.join("receiver");
        // Embed the fixture so Cargo tracks it and a cached test binary cannot
        // silently compile receiver source from another checkout or revision.
        let source = binary.with_extension("m");
        fs::write(&source, include_str!("receiver.m")).unwrap();
        let output = Command::new("clang")
            .args([
                "-fblocks",
                "-Wall",
                "-Wextra",
                "-Werror",
                "-framework",
                "AppKit",
            ])
            .args(["-framework", "ApplicationServices"])
            .arg(&source)
            .arg("-o")
            .arg(&binary)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let (sender, events) = mpsc::channel();
        let child = Command::new(&binary)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        // Establish cleanup immediately after spawn, before any operation that
        // can panic, including reader-thread creation or the ready handshake.
        let mut receiver = Self {
            child,
            events,
            files,
        };
        let stdout = receiver.child.stdout.take().unwrap();
        thread::spawn(move || {
            for line in BufReader::new(stdout).lines() {
                match line {
                    Ok(line) => {
                        // The parent deliberately drops this receiver on teardown.
                        if sender.send(line).is_err() {
                            return;
                        }
                    }
                    Err(error) => {
                        eprintln!("native receiver output failed: {error}");
                        return;
                    }
                }
            }
        });
        assert_eq!(
            receiver.events.recv_timeout(NATIVE_DEADLINE).unwrap(),
            "ready"
        );
        eprintln!("native_receiver_binary={}", binary.display());
        receiver
    }

    #[track_caller]
    fn pid(&self) -> i32 {
        self.child.id().try_into().unwrap()
    }

    #[track_caller]
    fn through_barrier(&self, sample: i64) -> Vec<(u16, i64)> {
        post_key(self.pid(), ANSI_S, sample);
        let mut received = Vec::new();
        loop {
            let line = self.events.recv_timeout(NATIVE_DEADLINE).unwrap();
            let mut parts = line.split(':');
            assert_eq!(parts.next(), Some("key"));
            let key = parts.next().unwrap().parse().unwrap();
            let received_sample = parts.next().unwrap().parse().unwrap();
            assert!(parts.next().is_none());
            received.push((key, received_sample));
            if (key, received_sample) == (ANSI_S, sample) {
                return received;
            }
        }
    }
}

#[track_caller]
fn post_key(pid: i32, key: u16, sample: i64) {
    for down in [true, false] {
        // SAFETY: NULL requests an independent event source. The owned event is
        // released after posting a copy, and is addressed only to our child PID.
        let event = unsafe { cg::CGEventCreateKeyboardEvent(null_mut(), key, down) };
        assert!(!event.is_null());
        let event = Owned(event);
        // SAFETY: This thread owns the valid event. No global/session event is
        // posted; the user-data sample lets the receiver identify FIFO barriers.
        unsafe {
            cg::CGEventSetFlags(event.0, EventFlags::empty());
            cg::CGEventSetIntegerValueField(event.0, EventField::KEYBOARD_EVENT_AUTOREPEAT, 0);
            cg::CGEventSetIntegerValueField(event.0, EventField::EVENT_SOURCE_USER_DATA, sample);
            cg::CGEventPostToPid(pid, event.0);
        }
    }
}

#[track_caller]
fn timeout_observation(observations: &Receiver<Observation>, attempted: bool, enabled: bool) {
    match observations.recv_timeout(NATIVE_DEADLINE).unwrap() {
        Observation::Notification {
            event_type,
            attempted: actual_attempt,
            enabled: actual_enabled,
        } => {
            assert_eq!(event_type, EventType::TAP_DISABLED_BY_TIMEOUT);
            assert_eq!(actual_attempt, attempted);
            assert_eq!(actual_enabled, enabled);
        }
        other => panic!("expected native timeout, got {other:?}"),
    }
}

#[test]
#[ignore = "requires an unlocked macOS desktop, existing Accessibility/Input Monitoring grants, Clang, and a disposable windowless AppKit receiver"]
fn native_process_tap_wiring_consumption_and_bounded_recovery() {
    for preference in [
        ConsumePreference::MustNotConsume,
        ConsumePreference::MustConsume,
    ] {
        crate::Hook::check_permission(preference)
            .expect("native checks require existing grants and never request access");
    }
    let receiver = NativeReceiver::new();
    for preference in [
        ConsumePreference::MustNotConsume,
        ConsumePreference::MustConsume,
    ] {
        let consuming = preference == ConsumePreference::MustConsume;
        let (observer, observations) = mpsc::channel();
        let state = Arc::new(State {
            dispatch: Dispatcher::new().unwrap(),
            stall_native: AtomicBool::new(false),
            observer: Some(observer),
        });
        // Same worker/context/source/callback as Hook::new; only the native
        // destination differs, confining synthetic keys and slow callbacks to
        // events addressed to the disposable receiver.
        let hook = Hook::with_state(preference, Target::Process(receiver.pid()), state).unwrap();
        let calls = Arc::new(AtomicUsize::new(0));
        let callback_calls = calls.clone();
        let (delivered, deliveries) = mpsc::channel();
        let (release, releases) = mpsc::channel();
        let hotkey = KeyCode::KeyA.with_modifiers(Modifiers::empty());
        hook.register(hotkey, move || {
            let index = callback_calls.fetch_add(1, Ordering::SeqCst);
            delivered.send(index).unwrap();
            if index == 0 {
                releases.recv_timeout(NATIVE_DEADLINE).unwrap();
            }
        })
        .unwrap();

        // Keep arbitrary user code blocked while another registered key and a
        // receiver barrier pass through the tap. Consuming taps must suppress
        // both A events immediately, without waiting for callback completion.
        post_key(receiver.pid(), ANSI_A, 0);
        assert_eq!(deliveries.recv_timeout(NATIVE_DEADLINE).unwrap(), 0);
        post_key(receiver.pid(), ANSI_A, 1);
        let received = receiver.through_barrier(1);
        assert_eq!(received.contains(&(ANSI_A, 0)), !consuming);
        assert_eq!(received.contains(&(ANSI_A, 1)), !consuming);
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        // Exceed the same delay that provokes a real native timeout below,
        // then prove a third key still passes through the live tap immediately.
        thread::sleep(NATIVE_STALL);
        post_key(receiver.pid(), ANSI_A, 2);
        assert_eq!(
            receiver.through_barrier(2).contains(&(ANSI_A, 2)),
            !consuming
        );
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        release.send(()).unwrap();
        assert_eq!(deliveries.recv_timeout(NATIVE_DEADLINE).unwrap(), 1);
        assert_eq!(deliveries.recv_timeout(NATIVE_DEADLINE).unwrap(), 2);
        receiver.through_barrier(10);
        assert!(matches!(
            observations.try_recv(),
            Err(mpsc::TryRecvError::Empty)
        ));

        if consuming {
            // Independently stall native plumbing to provoke real OS disable
            // notifications. Slow user callbacks above must never do this.
            hook.state.stall_native.store(true, Ordering::Relaxed);
            post_key(receiver.pid(), ANSI_A, 3);
            assert_eq!(deliveries.recv_timeout(NATIVE_DEADLINE).unwrap(), 3);
            // CoreGraphics may report the disable only when another event
            // reaches the tap. Drive that notification with the unregistered
            // barrier before waiting for the worker's recovery observation.
            receiver.through_barrier(3);
            timeout_observation(&observations, true, true);
            assert!(matches!(
                hook.register(hotkey, || {}),
                Err(crate::Error::AlreadyRegistered)
            ));

            // Delivery resumes through the same hook and registration, then a
            // second genuine timeout exhausts recovery instead of stalling input
            // indefinitely. Notifications must not increment the A callback.
            hook.state.stall_native.store(true, Ordering::Relaxed);
            post_key(receiver.pid(), ANSI_A, 4);
            assert_eq!(deliveries.recv_timeout(NATIVE_DEADLINE).unwrap(), 4);
            receiver.through_barrier(4);
            timeout_observation(&observations, false, false);
            assert_eq!(calls.load(Ordering::SeqCst), 5);

            post_key(receiver.pid(), ANSI_A, 5);
            assert!(receiver.through_barrier(5).contains(&(ANSI_A, 5)));
            assert_eq!(calls.load(Ordering::SeqCst), 5);
        }

        if !consuming {
            // Model an early native loop exit while the public owner remains
            // alive. Its retained control reference must survive worker teardown.
            // SAFETY: The hook owns the live loop and CF stop is thread-safe.
            unsafe { cf::CFRunLoopStop(hook.shutdown.event_loop.0) };
            assert!(matches!(
                observations.recv_timeout(NATIVE_DEADLINE).unwrap(),
                Observation::Stopped
            ));
            assert!(matches!(
                hook.register(hotkey, || {}),
                Err(crate::Error::Platform(
                    crate::macos::Error::ThreadStoppedUnexpectedly
                ))
            ));
            drop(hook);
        } else {
            drop(hook);
            assert!(matches!(
                observations.recv_timeout(NATIVE_DEADLINE).unwrap(),
                Observation::Stopped
            ));
        }
        eprintln!(
            "native_mode={} calls={}",
            if consuming { "consume" } else { "listen" },
            calls.load(Ordering::SeqCst)
        );
    }
    let directory = receiver.files.directory.clone();
    drop(receiver);
    assert!(
        !directory.exists(),
        "native fixture outputs must be removed"
    );
}

#[test]
fn retained_run_loop_outlives_its_originating_thread() {
    let handle = thread::spawn(|| RunLoop::current().unwrap())
        .join()
        .unwrap();
    // SAFETY: The retained handle owns its reference after the thread has exited.
    let mode = unsafe { cf::CFRunLoopCopyCurrentMode(handle.0) };
    assert!(mode.is_null(), "a loop that never ran has no active mode");
}

#[test]
fn stop_requested_before_run_is_not_lost() {
    let (sender, receiver) = mpsc::channel();
    let (start, starts) = mpsc::channel();
    let (stopped, stops) = mpsc::channel();
    thread::spawn(move || {
        let shutdown = Shutdown::new().unwrap();
        let event_loop = shutdown.event_loop.0;
        let source = shutdown.source.0;
        // SAFETY: The source and loop are retained, scheduled and removed on
        // this worker. The receiving owner retains both until after teardown.
        unsafe { cf::CFRunLoopAddSource(event_loop, source, cf::kCFRunLoopDefaultMode) };
        sender.send(shutdown).unwrap();
        starts.recv_timeout(NATIVE_DEADLINE).unwrap();
        // SAFETY: This is the worker's loop and the retained source is live.
        unsafe {
            cf::CFRunLoopRun();
            cf::CFRunLoopRemoveSource(event_loop, source, cf::kCFRunLoopDefaultMode);
        }
        stopped.send(()).unwrap();
    });
    let shutdown = receiver.recv_timeout(NATIVE_DEADLINE).unwrap();
    shutdown.request();
    start.send(()).unwrap();
    stops.recv_timeout(NATIVE_DEADLINE).unwrap();
}
