//! macOS privacy selection and explicit interaction, independent of native FFI.
//!
//! The backend supplies capability and grant observations. Silent checks never
//! prompt; an explicit request queries first and makes at most one native request.
//! No polling, remembered denial or process-global permission state is retained.

use crate::{ConsumePreference, Error, Permission, Result};

/// The existing CoreGraphics hook selection, shared by creation and permission checks.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TapMode {
    /// Observe keyboard input without filtering it.
    Listen,
    /// Filter keyboard events when a registered callback handles them.
    Consume,
}

impl TapMode {
    /// Preserve the existing native selection even for non-mandatory preferences.
    pub fn from_preference(consume: ConsumePreference) -> Self {
        match consume {
            ConsumePreference::PreferConsume | ConsumePreference::MustConsume => Self::Consume,
            ConsumePreference::NoPreference
            | ConsumePreference::PreferNoConsume
            | ConsumePreference::MustNotConsume => Self::Listen,
        }
    }

    // Apple separates passive listening from Accessibility-based filtering.
    // https://developer.apple.com/forums/thread/707680
    // Older macOS has no Input Monitoring API, so retain its existing AX grant.
    fn permission(self, input_monitoring_supported: bool) -> Permission {
        match self {
            Self::Listen if input_monitoring_supported => Permission::InputMonitoring,
            Self::Listen | Self::Consume => Permission::Accessibility,
        }
    }
}

/// A query and a user-authorized request must remain distinguishable at the FFI.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Interaction {
    /// Observe current access without UI.
    Check,
    /// A caller has admitted a user-requested permission interaction.
    Request,
}

/// One operation's native API capability and access calls; never a cached grant.
pub trait Backend {
    /// Whether the current OS exports the Input Monitoring API pair.
    fn input_monitoring_supported(&self) -> bool;
    /// Perform one native operation; failures must not become permission denials.
    fn access(&self, permission: Permission, interaction: Interaction) -> Result<bool>;
}

/// Query once, then request at most once when explicitly admitted. A denial is
/// returned with its typed requirement; it is never cached or retried here.
pub fn authorize(
    consume: ConsumePreference,
    interaction: Interaction,
    backend: &impl Backend,
) -> Result<()> {
    let permission =
        TapMode::from_preference(consume).permission(backend.input_monitoring_supported());

    if backend.access(permission, Interaction::Check)?
        || interaction == Interaction::Request
            && backend.access(permission, Interaction::Request)?
    {
        Ok(())
    } else {
        Err(Error::PermissionRequired(permission))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ConsumePreference, Error, Permission, Result,
        permission::macos::{Backend, Interaction, TapMode, authorize},
    };
    use core::cell::RefCell;

    struct Native {
        modern: bool,
        granted: bool,
        request_granted: bool,
        calls: RefCell<alloc::vec::Vec<(Permission, Interaction)>>,
    }

    impl Backend for Native {
        fn input_monitoring_supported(&self) -> bool {
            self.modern
        }

        fn access(&self, permission: Permission, interaction: Interaction) -> Result<bool> {
            self.calls.borrow_mut().push((permission, interaction));
            Ok(match interaction {
                Interaction::Check => self.granted,
                Interaction::Request => self.request_granted,
            })
        }
    }

    fn native(modern: bool, granted: bool, request_granted: bool) -> Native {
        Native {
            modern,
            granted,
            request_granted,
            calls: RefCell::default(),
        }
    }

    #[test]
    fn silent_construction_checks_the_selected_capability_without_requesting_it() {
        for (preference, expected_mode, expected_permission) in [
            (
                ConsumePreference::NoPreference,
                TapMode::Listen,
                Permission::InputMonitoring,
            ),
            (
                ConsumePreference::PreferNoConsume,
                TapMode::Listen,
                Permission::InputMonitoring,
            ),
            (
                ConsumePreference::MustNotConsume,
                TapMode::Listen,
                Permission::InputMonitoring,
            ),
            (
                ConsumePreference::PreferConsume,
                TapMode::Consume,
                Permission::Accessibility,
            ),
            (
                ConsumePreference::MustConsume,
                TapMode::Consume,
                Permission::Accessibility,
            ),
        ] {
            let backend = native(true, false, true);
            assert_eq!(TapMode::from_preference(preference), expected_mode);
            assert!(
                matches!(authorize(preference, Interaction::Check, &backend),
                Err(Error::PermissionRequired(permission)) if permission == expected_permission)
            );
            assert_eq!(
                *backend.calls.borrow(),
                [(expected_permission, Interaction::Check)]
            );
        }
    }

    #[test]
    fn explicit_requests_prompt_only_for_a_missing_grant_and_never_retry_in_a_loop() {
        for granted in [false, true] {
            for request_granted in [false, true] {
                let backend = native(true, granted, request_granted);
                let result = authorize(
                    ConsumePreference::PreferNoConsume,
                    Interaction::Request,
                    &backend,
                );
                assert_eq!(result.is_ok(), granted || request_granted);
                let calls = backend.calls.borrow();
                assert_eq!(calls[0], (Permission::InputMonitoring, Interaction::Check));
                assert_eq!(calls.len(), if granted { 1 } else { 2 });
                if !granted {
                    assert_eq!(
                        calls[1],
                        (Permission::InputMonitoring, Interaction::Request)
                    );
                }
            }
        }
    }

    #[test]
    fn older_macos_uses_its_existing_accessibility_permission_for_listening() {
        let backend = native(false, false, false);
        assert!(matches!(
            authorize(
                ConsumePreference::PreferNoConsume,
                Interaction::Request,
                &backend
            ),
            Err(Error::PermissionRequired(Permission::Accessibility))
        ));
        assert_eq!(
            *backend.calls.borrow(),
            [
                (Permission::Accessibility, Interaction::Check),
                (Permission::Accessibility, Interaction::Request),
            ]
        );
    }

    #[test]
    fn backend_errors_propagate_without_requesting_or_retrying_after_failure() {
        struct FailingNative {
            failure: Interaction,
            calls: RefCell<alloc::vec::Vec<Interaction>>,
        }

        impl Backend for FailingNative {
            fn input_monitoring_supported(&self) -> bool {
                true
            }

            fn access(&self, permission: Permission, interaction: Interaction) -> Result<bool> {
                assert_eq!(permission, Permission::InputMonitoring);
                self.calls.borrow_mut().push(interaction);
                if interaction == self.failure {
                    // Native platform errors differ by test host. This portable
                    // sentinel must likewise survive without becoming a denial.
                    Err(Error::UnmatchedPreference)
                } else {
                    Ok(false)
                }
            }
        }

        for (interaction, failure, expected_calls) in [
            (
                Interaction::Check,
                Interaction::Check,
                &[Interaction::Check][..],
            ),
            (
                Interaction::Request,
                Interaction::Check,
                &[Interaction::Check][..],
            ),
            (
                Interaction::Request,
                Interaction::Request,
                &[Interaction::Check, Interaction::Request][..],
            ),
        ] {
            let backend = FailingNative {
                failure,
                calls: RefCell::default(),
            };
            assert!(matches!(
                authorize(ConsumePreference::PreferNoConsume, interaction, &backend),
                Err(Error::UnmatchedPreference)
            ));
            assert_eq!(*backend.calls.borrow(), expected_calls);
        }
    }

    #[test]
    fn grants_are_queried_again_after_a_denial_or_revocation() {
        let mut backend = native(true, false, false);
        assert!(
            authorize(
                ConsumePreference::NoPreference,
                Interaction::Check,
                &backend
            )
            .is_err()
        );
        backend.granted = true;
        assert!(
            authorize(
                ConsumePreference::NoPreference,
                Interaction::Check,
                &backend
            )
            .is_ok()
        );
        backend.granted = false;
        assert!(
            authorize(
                ConsumePreference::NoPreference,
                Interaction::Check,
                &backend
            )
            .is_err()
        );
        assert_eq!(backend.calls.borrow().len(), 3);
    }
}
