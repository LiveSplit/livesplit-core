//! The general settings of the layout that apply to all components.

use super::{alloc, own_drop};
use livesplit_core::GeneralLayoutSettings;

/// type
pub type OwnedGeneralLayoutSettings = *mut GeneralLayoutSettings;

/// Creates a default general layout settings configuration.
#[no_mangle]
pub unsafe extern "C" fn GeneralLayoutSettings_default() -> OwnedGeneralLayoutSettings {
    alloc(GeneralLayoutSettings::default())
}

/// drop
#[no_mangle]
pub unsafe extern "C" fn GeneralLayoutSettings_drop(this: OwnedGeneralLayoutSettings) {
    own_drop(this);
}
