//! The general settings of the layout that apply to all components.

use livesplit_core::GeneralLayoutSettings;

/// type
pub type OwnedGeneralLayoutSettings = Box<GeneralLayoutSettings>;

/// Creates a default general layout settings configuration.
#[unsafe(no_mangle)]
pub extern "C" fn GeneralLayoutSettings_default() -> OwnedGeneralLayoutSettings {
    Default::default()
}

/// drop
#[unsafe(no_mangle)]
pub extern "C" fn GeneralLayoutSettings_drop(this: OwnedGeneralLayoutSettings) {
    drop(this);
}
