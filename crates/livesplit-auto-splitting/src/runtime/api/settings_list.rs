use anyhow::{format_err, Result};
use slotmap::{Key, KeyData};
use wasmtime::{Caller, Linker};

use crate::{
    runtime::{Context, SettingValueKey, SettingsListKey},
    settings, CreationError, Timer,
};

pub fn bind<T: Timer>(linker: &mut Linker<Context<T>>) -> Result<(), CreationError> {
    linker
        .func_wrap("env", "settings_list_new", {
            |mut caller: Caller<'_, Context<T>>| {
                let ctx = caller.data_mut();
                ctx.settings_lists
                    .insert(settings::List::new())
                    .data()
                    .as_ffi()
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "settings_list_new",
        })?
        .func_wrap("env", "settings_list_free", {
            |mut caller: Caller<'_, Context<T>>, settings_list: u64| {
                caller
                    .data_mut()
                    .settings_lists
                    .remove(SettingsListKey::from(KeyData::from_ffi(settings_list)))
                    .ok_or_else(|| format_err!("Invalid settings list handle: {settings_list}"))?;
                Ok(())
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "settings_list_free",
        })?
        .func_wrap("env", "settings_list_copy", {
            |mut caller: Caller<'_, Context<T>>, settings_list: u64| {
                let ctx = caller.data_mut();

                let settings_list = ctx
                    .settings_lists
                    .get(SettingsListKey::from(KeyData::from_ffi(settings_list)))
                    .ok_or_else(|| format_err!("Invalid settings list handle: {settings_list}"))?
                    .clone();

                Ok(ctx.settings_lists.insert(settings_list).data().as_ffi())
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "settings_list_copy",
        })?
        .func_wrap("env", "settings_list_len", {
            |caller: Caller<'_, Context<T>>, settings_list: u64| {
                let ctx = caller.data();

                let len = ctx
                    .settings_lists
                    .get(SettingsListKey::from(KeyData::from_ffi(settings_list)))
                    .ok_or_else(|| format_err!("Invalid settings list handle: {settings_list}"))?
                    .len()
                    .try_into()
                    .unwrap_or(u64::MAX);

                Ok(len)
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "settings_list_len",
        })?
        .func_wrap("env", "settings_list_get", {
            |mut caller: Caller<'_, Context<T>>, settings_list: u64, index: u64| {
                let ctx = caller.data_mut();

                let maybe_value = if let Ok(index) = index.try_into() {
                    ctx.settings_lists
                        .get(SettingsListKey::from(KeyData::from_ffi(settings_list)))
                        .ok_or_else(|| {
                            format_err!("Invalid settings list handle: {settings_list}")
                        })?
                        .get(index)
                } else {
                    None
                };

                Ok(if let Some(value) = maybe_value {
                    ctx.setting_values.insert(value.clone()).data().as_ffi()
                } else {
                    0
                })
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "settings_list_get",
        })?
        .func_wrap("env", "settings_list_push", {
            |mut caller: Caller<'_, Context<T>>, settings_list: u64, setting_value: u64| {
                let context = caller.data_mut();

                let settings_list = context
                    .settings_lists
                    .get_mut(SettingsListKey::from(KeyData::from_ffi(settings_list)))
                    .ok_or_else(|| format_err!("Invalid settings list handle: {settings_list}"))?;

                let setting_value = context
                    .setting_values
                    .get(SettingValueKey::from(KeyData::from_ffi(setting_value)))
                    .ok_or_else(|| format_err!("Invalid setting value handle: {setting_value}"))?;

                settings_list.push(setting_value.clone());

                Ok(())
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "settings_list_push",
        })?
        .func_wrap("env", "settings_list_insert", {
            |mut caller: Caller<'_, Context<T>>,
             settings_list: u64,
             index: u64,
             setting_value: u64| {
                let context = caller.data_mut();

                let settings_list = context
                    .settings_lists
                    .get_mut(SettingsListKey::from(KeyData::from_ffi(settings_list)))
                    .ok_or_else(|| format_err!("Invalid settings list handle: {settings_list}"))?;

                let setting_value = context
                    .setting_values
                    .get(SettingValueKey::from(KeyData::from_ffi(setting_value)))
                    .ok_or_else(|| format_err!("Invalid setting value handle: {setting_value}"))?;

                Ok(settings_list
                    .insert(
                        index.try_into().unwrap_or(usize::MAX),
                        setting_value.clone(),
                    )
                    .is_ok() as u32)
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "settings_list_insert",
        })?;
    Ok(())
}
