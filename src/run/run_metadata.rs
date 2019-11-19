use crate::indexmap::map::{IndexMap, Iter};
use serde::{Deserialize, Serialize};
use crate::platform::prelude::*;

/// The Run Metadata stores additional information about a run, like the
/// platform and region of the game. All of this information is optional.
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunMetadata {
    /// The speedrun.com Run ID of the run. You need to ensure that the record
    /// on speedrun.com matches up with the Personal Best of this run. This may
    /// be empty if there's no association.
    pub run_id: String,
    /// The name of the platform this game is run on. This may be empty if it's
    /// not specified.
    pub platform_name: String,
    /// Specifies whether this speedrun is done on an emulator. Keep in mind
    /// that `false` may also mean that this information is simply not known.
    pub uses_emulator: bool,
    /// The name of the region this game is from. This may be empty if it's not
    /// specified.
    pub region_name: String,
    /// Stores all the variables. A variable is an arbitrary key value pair
    /// storing additional information about the category. An example of this
    /// may be whether Amiibos are used in this category.
    pub variables: IndexMap<String, String>,
}

impl RunMetadata {
    /// Creates a new empty Run Metadata.
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Accesses the speedrun.com Run ID of the run. This Run ID specify which
    /// Record on speedrun.com this run is associated with. This should be
    /// changed once the Personal Best doesn't match up with that record
    /// anymore. This may be empty if there's no association.
    #[inline]
    pub fn run_id(&self) -> &str {
        &self.run_id
    }

    /// Sets the speedrun.com Run ID of the run. You need to ensure that the
    /// record on speedrun.com matches up with the Personal Best of this run.
    /// This may be empty if there's no association.
    #[inline]
    pub fn set_run_id<S>(&mut self, id: S)
    where
        S: AsRef<str>,
    {
        self.run_id.clear();
        self.run_id.push_str(id.as_ref());
    }

    /// Accesses the name of the platform this game is run on. This may be empty
    /// if it's not specified.
    #[inline]
    pub fn platform_name(&self) -> &str {
        &self.platform_name
    }

    /// Sets the name of the platform this game is run on. This may be empty if
    /// it's not specified.
    #[inline]
    pub fn set_platform_name<S>(&mut self, name: S)
    where
        S: AsRef<str>,
    {
        self.platform_name.clear();
        self.platform_name.push_str(name.as_ref());
    }

    /// Returns `true` if this speedrun is done on an emulator. However `false`
    /// may also indicate that this information is simply not known.
    #[inline]
    pub fn uses_emulator(&self) -> bool {
        self.uses_emulator
    }

    /// Specifies whether this speedrun is done on an emulator. Keep in mind
    /// that `false` may also mean that this information is simply not known.
    #[inline]
    pub fn set_emulator_usage(&mut self, uses_emulator: bool) {
        self.uses_emulator = uses_emulator;
    }

    /// Accesses the name of the region this game is from. This may be empty if
    /// it's not specified.
    #[inline]
    pub fn region_name(&self) -> &str {
        &self.region_name
    }

    /// Sets the name of the region this game is from. This may be empty if it's
    /// not specified.
    #[inline]
    pub fn set_region_name<S>(&mut self, region_name: S)
    where
        S: AsRef<str>,
    {
        self.region_name.clear();
        self.region_name.push_str(region_name.as_ref());
    }

    /// Sets the variable with the name specified to the value specified. A
    /// variable is an arbitrary key value pair storing additional information
    /// about the category. An example of this may be whether Amiibos are used
    /// in this category. If the variable doesn't exist yet, it is being
    /// inserted.
    pub fn set_variable<N, V>(&mut self, name: N, value: V)
    where
        N: Into<String>,
        V: Into<String>,
    {
        self.variables.insert(name.into(), value.into());
    }

    /// Removes the variable with the name specified.
    pub fn remove_variable<S>(&mut self, name: S)
    where
        S: AsRef<str>,
    {
        self.variables.shift_remove(name.as_ref());
    }

    /// Returns an iterator iterating over all the variables and their values
    /// that have been specified.
    pub fn variables(&self) -> Iter<'_, String, String> {
        self.variables.iter()
    }

    /// Resets all the Metadata Information.
    pub fn clear(&mut self) {
        self.run_id.clear();
        self.platform_name.clear();
        self.region_name.clear();
        self.uses_emulator = false;
        self.variables.clear();
    }
}
