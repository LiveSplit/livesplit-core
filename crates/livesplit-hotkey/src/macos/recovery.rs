//! Routes tap notifications before keyboard access and bounds native recovery.
//!
//! This module owns no native port, clock or permissions. A worker keeps one
//! recovery state for the lifetime of its tap and injects both native recovery
//! and keyboard dispatch. Control notifications never call keyboard dispatch.

use crate::macos::cg::EventType;
use std::cell::Cell;

#[derive(Clone, Copy)]
enum State {
    Available,
    Exhausted,
}

/// One automatic timeout retry for one native tap's complete lifetime.
///
/// Apple distinguishes unresponsive taps from user-requested disables:
/// https://developer.apple.com/documentation/coregraphics/cgevent/tapenable(tap:enable:)
/// Respect the latter. A transient stall can recover once, but repeatedly
/// restoring a stalled native worker could repeatedly stall session input.
/// Do not replenish the retry on key delivery: a stalled worker can still deliver
/// keys between timeouts. Recreating the hook is an explicit caller decision.
pub(crate) struct Recovery(Cell<State>);

impl Recovery {
    pub(crate) const fn new() -> Self {
        Self(Cell::new(State::Available))
    }

    /// Returns the original event except when a real key reaches its handler.
    ///
    /// Only control notifications read or write recovery state. Ordinary keys
    /// add classification and a null check, with no allocation or clock read.
    /// Injected operations are statically dispatched and need no boxed callback.
    /// The retry is spent before invoking native code, including if that call
    /// fails to restore delivery or reenters notification handling.
    pub(crate) fn dispatch<T>(
        &self,
        event_type: EventType,
        has_event: bool,
        original: T,
        reenable: impl FnOnce(),
        key_down: impl FnOnce() -> T,
    ) -> T {
        match event_type {
            EventType::TAP_DISABLED_BY_TIMEOUT => {
                if matches!(self.0.replace(State::Exhausted), State::Available) {
                    reenable();
                }
            }
            EventType::TAP_DISABLED_BY_USER_INPUT => self.0.set(State::Exhausted),
            EventType::KEY_DOWN if has_event => return key_down(),
            _ => {}
        }

        original
    }
}

#[cfg(test)]
mod tests {
    use crate::macos::{cg::EventType, recovery::Recovery};
    use std::cell::Cell;

    #[test]
    fn notifications_never_read_or_dispatch_keyboard_events() {
        for event_type in [
            EventType::TAP_DISABLED_BY_TIMEOUT,
            EventType::TAP_DISABLED_BY_USER_INPUT,
        ] {
            for has_event in [false, true] {
                let recovery = Recovery::new();
                let attempts = Cell::new(0);
                assert_eq!(
                    recovery.dispatch(
                        event_type,
                        has_event,
                        7,
                        || attempts.set(attempts.get() + 1),
                        || panic!("a control notification must never read keyboard fields"),
                    ),
                    7,
                );
                assert_eq!(
                    attempts.get(),
                    usize::from(event_type == EventType::TAP_DISABLED_BY_TIMEOUT)
                );
            }
        }
    }

    #[test]
    fn timeouts_get_one_attempt_even_after_successful_key_delivery() {
        let recovery = Recovery::new();
        let attempts = Cell::new(0);
        for _ in 0..3 {
            recovery.dispatch(
                EventType::TAP_DISABLED_BY_TIMEOUT,
                false,
                (),
                || attempts.set(attempts.get() + 1),
                || panic!("timeouts are not keys"),
            );
            assert_eq!(
                recovery.dispatch(
                    EventType::KEY_DOWN,
                    true,
                    7,
                    || panic!("a key must not reenable a tap"),
                    || 9,
                ),
                9,
            );
        }
        assert_eq!(attempts.get(), 1);
    }

    #[test]
    fn user_disable_exhausts_recovery_before_a_later_timeout() {
        let recovery = Recovery::new();
        for event_type in [
            EventType::TAP_DISABLED_BY_USER_INPUT,
            EventType::TAP_DISABLED_BY_TIMEOUT,
        ] {
            recovery.dispatch(
                event_type,
                false,
                (),
                || panic!("a user-disabled tap must stay disabled"),
                || panic!("notifications are not keys"),
            );
        }
    }

    #[test]
    fn retry_is_spent_before_a_reentrant_native_operation() {
        let recovery = Recovery::new();
        recovery.dispatch(
            EventType::TAP_DISABLED_BY_TIMEOUT,
            false,
            (),
            || {
                recovery.dispatch(
                    EventType::TAP_DISABLED_BY_TIMEOUT,
                    false,
                    (),
                    || panic!("reentrant failure must not retry again"),
                    || panic!("notifications are not keys"),
                );
            },
            || panic!("notifications are not keys"),
        );
    }

    #[test]
    fn unrelated_and_missing_events_do_not_spend_recovery() {
        let recovery = Recovery::new();
        for (event_type, has_event) in [
            (EventType::NULL, true),
            (EventType::KEY_UP, true),
            (EventType::FLAGS_CHANGED, true),
            (EventType::KEY_DOWN, false),
        ] {
            assert_eq!(
                recovery.dispatch(
                    event_type,
                    has_event,
                    7,
                    || panic!("ordinary messages must not reenable a tap"),
                    || panic!("only a present key-down event may reach keyboard access"),
                ),
                7,
            );
        }

        let retried = Cell::new(false);
        recovery.dispatch(
            EventType::TAP_DISABLED_BY_TIMEOUT,
            false,
            (),
            || retried.set(true),
            || panic!("notifications are not keys"),
        );
        assert!(retried.get());
    }
}
