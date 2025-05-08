use anyhow::{Result, format_err};
use slotmap::{Key, KeyData};
use wasmtime::{Caller, Linker};

use crate::{
    CreationError, Timer,
    runtime::{Context, SettingValueKey, SettingsMapKey},
    settings,
};

use super::{get_arr_mut, get_slice_mut, get_str, memory_and_context};

pub fn bind<T: Timer>(linker: &mut Linker<Context<T>>) -> Result<(), CreationError> {
    linker
        .func_wrap("env", "settings_map_new", {
            |mut caller: Caller<'_, Context<T>>| {
                let ctx = caller.data_mut();
                ctx.settings_maps
                    .insert(settings::Map::new())
                    .data()
                    .as_ffi()
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "settings_map_new",
        })?
        .func_wrap("env", "settings_map_free", {
            |mut caller: Caller<'_, Context<T>>, settings_map: u64| {
                caller
                    .data_mut()
                    .settings_maps
                    .remove(SettingsMapKey::from(KeyData::from_ffi(settings_map)))
                    .ok_or_else(|| format_err!("Invalid settings map handle: {settings_map}"))?;
                Ok(())
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "settings_map_free",
        })?
        .func_wrap("env", "settings_map_load", {
            |mut caller: Caller<'_, Context<T>>| {
                let ctx = caller.data_mut();
                let settings_map = ctx.shared_data.get_settings_map();
                ctx.settings_maps.insert(settings_map).data().as_ffi()
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "settings_map_load",
        })?
        .func_wrap("env", "settings_map_store", {
            |mut caller: Caller<'_, Context<T>>, settings_map: u64| {
                let ctx = caller.data_mut();

                let settings_map = ctx
                    .settings_maps
                    .get(SettingsMapKey::from(KeyData::from_ffi(settings_map)))
                    .ok_or_else(|| format_err!("Invalid settings map handle: {settings_map}"))?
                    .clone();

                ctx.shared_data.set_settings_map(settings_map);

                Ok(())
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "settings_map_store",
        })?
        .func_wrap("env", "settings_map_store_if_unchanged", {
            |mut caller: Caller<'_, Context<T>>, old_settings_map: u64, new_settings_map: u64| {
                let ctx = caller.data_mut();

                let old_settings_map = ctx
                    .settings_maps
                    .get(SettingsMapKey::from(KeyData::from_ffi(old_settings_map)))
                    .ok_or_else(|| {
                        format_err!("Invalid old settings map handle: {old_settings_map}")
                    })?;

                let new_settings_map = ctx
                    .settings_maps
                    .get(SettingsMapKey::from(KeyData::from_ffi(new_settings_map)))
                    .ok_or_else(|| {
                        format_err!("Invalid new settings map handle: {new_settings_map}")
                    })?
                    .clone();

                let success = ctx
                    .shared_data
                    .set_settings_map_if_unchanged(old_settings_map, new_settings_map);

                Ok(success as u32)
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "settings_map_store_if_unchanged",
        })?
        .func_wrap("env", "settings_map_copy", {
            |mut caller: Caller<'_, Context<T>>, settings_map: u64| {
                let ctx = caller.data_mut();

                let settings_map = ctx
                    .settings_maps
                    .get(SettingsMapKey::from(KeyData::from_ffi(settings_map)))
                    .ok_or_else(|| format_err!("Invalid settings map handle: {settings_map}"))?
                    .clone();

                Ok(ctx.settings_maps.insert(settings_map).data().as_ffi())
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "settings_map_copy",
        })?
        .func_wrap("env", "settings_map_insert", {
            |mut caller: Caller<'_, Context<T>>,
             settings_map: u64,
             key_ptr: u32,
             key_len: u32,
             setting_value: u64| {
                let (memory, context) = memory_and_context(&mut caller);

                let settings_map = context
                    .settings_maps
                    .get_mut(SettingsMapKey::from(KeyData::from_ffi(settings_map)))
                    .ok_or_else(|| format_err!("Invalid settings map handle: {settings_map}"))?;

                let setting_value = context
                    .setting_values
                    .get(SettingValueKey::from(KeyData::from_ffi(setting_value)))
                    .ok_or_else(|| format_err!("Invalid setting value handle: {setting_value}"))?;

                let key = get_str(memory, key_ptr, key_len)?;

                settings_map.insert(key.into(), setting_value.clone());

                Ok(())
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "settings_map_insert",
        })?
        .func_wrap("env", "settings_map_get", {
            |mut caller: Caller<'_, Context<T>>, settings_map: u64, key_ptr: u32, key_len: u32| {
                let (memory, context) = memory_and_context(&mut caller);

                let settings_map = context
                    .settings_maps
                    .get(SettingsMapKey::from(KeyData::from_ffi(settings_map)))
                    .ok_or_else(|| format_err!("Invalid settings map handle: {settings_map}"))?;

                let key = get_str(memory, key_ptr, key_len)?;

                Ok(match settings_map.get(key) {
                    Some(value) => context.setting_values.insert(value.clone()).data().as_ffi(),
                    None => 0,
                })
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "settings_map_get",
        })?
        .func_wrap("env", "settings_map_len", {
            |mut caller: Caller<'_, Context<T>>, settings_map: u64| {
                let ctx = caller.data_mut();

                let len = ctx
                    .settings_maps
                    .get(SettingsMapKey::from(KeyData::from_ffi(settings_map)))
                    .ok_or_else(|| format_err!("Invalid settings map handle: {settings_map}"))?
                    .len()
                    .try_into()
                    .unwrap_or(u64::MAX);

                Ok(len)
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "settings_map_len",
        })?
        .func_wrap("env", "settings_map_get_key_by_index", {
            |mut caller: Caller<'_, Context<T>>,
             settings_map: u64,
             index: u64,
             buf_ptr: u32,
             buf_len_ptr: u32| {
                let (memory, context) = memory_and_context(&mut caller);

                let settings_map = context
                    .settings_maps
                    .get(SettingsMapKey::from(KeyData::from_ffi(settings_map)))
                    .ok_or_else(|| format_err!("Invalid settings map handle: {settings_map}"))?;

                let len_bytes = get_arr_mut(memory, buf_len_ptr)?;

                let slot = settings_map.get_by_index(index.try_into().unwrap_or(usize::MAX));

                if let Some((key, _)) = slot {
                    let len = u32::from_le_bytes(*len_bytes) as usize;
                    *len_bytes = (key.len() as u32).to_le_bytes();

                    if len < key.len() {
                        return Ok(0u32);
                    }
                    let buf = get_slice_mut(memory, buf_ptr, key.len() as _)?;
                    buf.copy_from_slice(key.as_bytes());
                    Ok(1u32)
                } else {
                    *len_bytes = 0u32.to_le_bytes();
                    Ok(0u32)
                }
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "settings_map_get_key_by_index",
        })?
        .func_wrap("env", "settings_map_get_value_by_index", {
            |mut caller: Caller<'_, Context<T>>, settings_map: u64, index: u64| {
                let ctx = caller.data_mut();

                let maybe_slot = if let Ok(index) = index.try_into() {
                    ctx.settings_maps
                        .get(SettingsMapKey::from(KeyData::from_ffi(settings_map)))
                        .ok_or_else(|| format_err!("Invalid settings map handle: {settings_map}"))?
                        .get_by_index(index)
                } else {
                    None
                };

                Ok(if let Some((_, value)) = maybe_slot {
                    ctx.setting_values.insert(value.clone()).data().as_ffi()
                } else {
                    0
                })
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "settings_map_get_value_by_index",
        })?;
    Ok(())
}
