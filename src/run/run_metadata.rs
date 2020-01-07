use crate::indexmap::map::{IndexMap, Iter};
use crate::platform::prelude::*;
use serde::{Deserialize, Serialize};

/// A custom variable is a key value pair storing additional information about a
/// run. Unlike the speedrun.com variables, these can be fully custom and don't
/// need to correspond to anything on speedrun.com. Permanent custom variables
/// can be specified by the runner. Additionally auto splitters or other sources
/// may provide temporary custom variables that are not stored in the splits
/// files.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CustomVariable {
    /// The current value of the custom variable. This may be provided by the
    /// runner in the run editor or it may be provided through other means such
    /// as an auto splitter.
    pub value: String,
    /// States whether the variable is permanent. Temporary variables don't get
    /// stored in splits files. They also don't get shown in the run editor.
    pub is_permanent: bool,
}

impl CustomVariable {
    /// Turns the custom variable into a permanent one. If it already is
    /// permanent variable, nothing happens.
    pub fn permanent(&mut self) -> &mut Self {
        self.is_permanent = true;
        self
    }

    /// Sets the value of the custom variable.
    pub fn set_value<S>(&mut self, value: S)
    where
        S: AsRef<str>,
    {
        self.value.clear();
        self.value.push_str(value.as_ref());
    }

    /// Clears the value of the custom variable. This does not delete the custom
    /// variable.
    pub fn clear_value(&mut self) {
        self.value.clear();
    }
}

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
    /// Stores all the speedrun.com variables. A speedrun.com variable is a key
    /// value pair storing additional information about the category. An example
    /// of this may be whether Amiibos are used in this category. Use a custom
    /// variable for storing arbitrary key value pairs that are independent of
    /// speedrun.com.
    pub speedrun_com_variables: IndexMap<String, String>,
    /// Stores all the custom variables. A custom variable is a key value pair
    /// storing additional information about a run. Unlike the speedrun.com
    /// variables, these can be fully custom and don't need to correspond to
    /// anything on speedrun.com. Permanent custom variables can be specified by
    /// the runner. Additionally auto splitters or other sources may provide
    /// temporary custom variables that are not stored in the splits files.
    pub custom_variables: IndexMap<String, CustomVariable>,
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

    /// Sets the speedrun.com variable with the name specified to the value
    /// specified. A speedrun.com variable is an arbitrary key value pair
    /// storing additional information about the category. An example of this
    /// may be whether Amiibos are used in this category. If the variable
    /// doesn't exist yet, it is being inserted.
    pub fn set_speedrun_com_variable<N, V>(&mut self, name: N, value: V)
    where
        N: Into<String>,
        V: Into<String>,
    {
        self.speedrun_com_variables
            .insert(name.into(), value.into());
    }

    /// Removes the speedrun.com variable with the name specified.
    pub fn remove_speedrun_com_variable<S>(&mut self, name: S)
    where
        S: AsRef<str>,
    {
        self.speedrun_com_variables.shift_remove(name.as_ref());
    }

    /// Returns an iterator iterating over all the speedrun.com variables and
    /// their values that have been specified.
    pub fn speedrun_com_variables(&self) -> Iter<'_, String, String> {
        self.speedrun_com_variables.iter()
    }

    /// Accesses the custom variable with the name specified if there is one.
    pub fn custom_variable<S>(&self, name: S) -> Option<&CustomVariable>
    where
        S: AsRef<str>,
    {
        self.custom_variables.get(name.as_ref())
    }

    /// Accesses the value of the custom variable with the name specified if
    /// there is one.
    pub fn custom_variable_value<S>(&self, name: S) -> Option<&str>
    where
        S: AsRef<str>,
    {
        Some(&self.custom_variable(name)?.value)
    }

    /// Mutably accesses the custom variable with the name specified. If the
    /// variable does not exist, it gets created as a temporary variable.
    pub fn custom_variable_mut<S>(&mut self, name: S) -> &mut CustomVariable
    where
        S: Into<String>,
    {
        self.custom_variables.entry(name.into()).or_default()
    }

    /// Removes the custom variable with the name specified. Nothing happens if
    /// the variable does not exist.
    pub fn remove_custom_variable<S>(&mut self, name: S)
    where
        S: AsRef<str>,
    {
        self.custom_variables.shift_remove(name.as_ref());
    }

    /// Returns an iterator iterating over all the custom variables and their
    /// values. This includes both temporary and permanent variables.
    pub fn custom_variables(&self) -> Iter<'_, String, CustomVariable> {
        self.custom_variables.iter()
    }

    /// Resets all the Metadata Information.
    pub fn clear(&mut self) {
        self.run_id.clear();
        self.platform_name.clear();
        self.region_name.clear();
        self.uses_emulator = false;
        self.speedrun_com_variables.clear();
        self.custom_variables.clear();
    }
}
