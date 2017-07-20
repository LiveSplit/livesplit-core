use livesplit_core::GeneralLayoutSettings;
use super::{alloc, own_drop};

pub type OwnedGeneralLayoutSettings = *mut GeneralLayoutSettings;

#[no_mangle]
pub unsafe extern "C" fn GeneralLayoutSettings_default() -> OwnedGeneralLayoutSettings {
    alloc(GeneralLayoutSettings::default())
}

#[no_mangle]
pub unsafe extern "C" fn GeneralLayoutSettings_drop(this: OwnedGeneralLayoutSettings) {
    own_drop(this);
}
