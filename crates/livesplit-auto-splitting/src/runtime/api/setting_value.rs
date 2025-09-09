use anyhow::{Result, format_err};
use slotmap::{Key, KeyData};
use wasmtime::{Caller, Linker};

use crate::{
    CreationError, Timer,
    runtime::{Context, SettingValueKey, SettingsListKey, SettingsMapKey},
    settings,
};

use super::{get_arr_mut, get_slice_mut, get_str, memory_and_context};

pub fn bind<T: Timer>(linker: &mut Linker<Context<T>>) -> Result<(), CreationError> {
    linker
        .func_wrap("env", "setting_value_new_map", {
            |mut caller: Caller<Context<T>>, settings_map: u64| {
                let context = caller.data_mut();

                let settings_map = context
                    .settings_maps
                    .get(SettingsMapKey::from(KeyData::from_ffi(settings_map)))
                    .ok_or_else(|| format_err!("Invalid settings map handle: {settings_map}"))?;

                Ok(context
                    .setting_values
                    .insert(settings::Value::Map(settings_map.clone()))
                    .data()
                    .as_ffi())
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "setting_value_new_map",
        })?
        .func_wrap("env", "setting_value_new_list", {
            |mut caller: Caller<Context<T>>, settings_list: u64| {
                let context = caller.data_mut();

                let settings_list = context
                    .settings_lists
                    .get(SettingsListKey::from(KeyData::from_ffi(settings_list)))
                    .ok_or_else(|| format_err!("Invalid settings list handle: {settings_list}"))?;

                Ok(context
                    .setting_values
                    .insert(settings::Value::List(settings_list.clone()))
                    .data()
                    .as_ffi())
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "setting_value_new_list",
        })?
        .func_wrap("env", "setting_value_new_bool", {
            |mut caller: Caller<Context<T>>, value: u32| {
                Ok(caller
                    .data_mut()
                    .setting_values
                    .insert(settings::Value::Bool(value != 0))
                    .data()
                    .as_ffi())
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "setting_value_new_bool",
        })?
        .func_wrap("env", "setting_value_new_i64", {
            |mut caller: Caller<Context<T>>, value: i64| {
                Ok(caller
                    .data_mut()
                    .setting_values
                    .insert(settings::Value::I64(value))
                    .data()
                    .as_ffi())
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "setting_value_new_i64",
        })?
        .func_wrap("env", "setting_value_new_f64", {
            |mut caller: Caller<Context<T>>, value: f64| {
                Ok(caller
                    .data_mut()
                    .setting_values
                    .insert(settings::Value::F64(value))
                    .data()
                    .as_ffi())
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "setting_value_new_f64",
        })?
        .func_wrap("env", "setting_value_new_string", {
            |mut caller: Caller<Context<T>>, ptr: u32, len: u32| {
                let (memory, context) = memory_and_context(&mut caller);
                let value = get_str(memory, ptr, len)?;
                Ok(context
                    .setting_values
                    .insert(settings::Value::String(value.into()))
                    .data()
                    .as_ffi())
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "setting_value_new_string",
        })?
        .func_wrap("env", "setting_value_free", {
            |mut caller: Caller<Context<T>>, setting_value: u64| {
                caller
                    .data_mut()
                    .setting_values
                    .remove(SettingValueKey::from(KeyData::from_ffi(setting_value)))
                    .ok_or_else(|| format_err!("Invalid setting value handle: {setting_value}"))?;
                Ok(())
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "setting_value_free",
        })?
        .func_wrap("env", "setting_value_copy", {
            |mut caller: Caller<Context<T>>, setting_value: u64| {
                let ctx = caller.data_mut();

                let setting_value = ctx
                    .setting_values
                    .get(SettingValueKey::from(KeyData::from_ffi(setting_value)))
                    .ok_or_else(|| format_err!("Invalid settings value handle: {setting_value}"))?
                    .clone();

                Ok(ctx.setting_values.insert(setting_value).data().as_ffi())
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "setting_value_copy",
        })?
        .func_wrap("env", "setting_value_get_type", {
            |mut caller: Caller<Context<T>>, setting_value: u64| -> Result<u32> {
                let (_, context) = memory_and_context(&mut caller);

                let setting_value = context
                    .setting_values
                    .get(SettingValueKey::from(KeyData::from_ffi(setting_value)))
                    .ok_or_else(|| format_err!("Invalid setting value handle: {setting_value}"))?;

                Ok(match setting_value {
                    settings::Value::Map(_) => 1,
                    settings::Value::List(_) => 2,
                    settings::Value::Bool(_) => 3,
                    settings::Value::I64(_) => 4,
                    settings::Value::F64(_) => 5,
                    settings::Value::String(_) => 6,
                })
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "setting_value_get_type",
        })?
        .func_wrap("env", "setting_value_get_map", {
            |mut caller: Caller<Context<T>>, setting_value: u64, value_ptr: u32| {
                let (memory, context) = memory_and_context(&mut caller);

                let setting_value = context
                    .setting_values
                    .get(SettingValueKey::from(KeyData::from_ffi(setting_value)))
                    .ok_or_else(|| format_err!("Invalid setting value handle: {setting_value}"))?;

                let value_ptr = get_arr_mut(memory, value_ptr)?;

                if let settings::Value::Map(value) = setting_value {
                    *value_ptr = context
                        .settings_maps
                        .insert(value.clone())
                        .data()
                        .as_ffi()
                        .to_le_bytes();
                    Ok(1u32)
                } else {
                    Ok(0u32)
                }
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "setting_value_get_map",
        })?
        .func_wrap("env", "setting_value_get_list", {
            |mut caller: Caller<Context<T>>, setting_value: u64, value_ptr: u32| {
                let (memory, context) = memory_and_context(&mut caller);

                let setting_value = context
                    .setting_values
                    .get(SettingValueKey::from(KeyData::from_ffi(setting_value)))
                    .ok_or_else(|| format_err!("Invalid setting value handle: {setting_value}"))?;

                let value_ptr = get_arr_mut(memory, value_ptr)?;

                if let settings::Value::List(value) = setting_value {
                    *value_ptr = context
                        .settings_lists
                        .insert(value.clone())
                        .data()
                        .as_ffi()
                        .to_le_bytes();
                    Ok(1u32)
                } else {
                    Ok(0u32)
                }
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "setting_value_get_list",
        })?
        .func_wrap("env", "setting_value_get_bool", {
            |mut caller: Caller<Context<T>>, setting_value: u64, value_ptr: u32| {
                let (memory, context) = memory_and_context(&mut caller);

                let setting_value = context
                    .setting_values
                    .get(SettingValueKey::from(KeyData::from_ffi(setting_value)))
                    .ok_or_else(|| format_err!("Invalid setting value handle: {setting_value}"))?;

                let [out] = get_arr_mut(memory, value_ptr)?;

                if let settings::Value::Bool(value) = setting_value {
                    *out = *value as u8;
                    Ok(1u32)
                } else {
                    Ok(0u32)
                }
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "setting_value_get_bool",
        })?
        .func_wrap("env", "setting_value_get_i64", {
            |mut caller: Caller<Context<T>>, setting_value: u64, value_ptr: u32| {
                let (memory, context) = memory_and_context(&mut caller);

                let setting_value = context
                    .setting_values
                    .get(SettingValueKey::from(KeyData::from_ffi(setting_value)))
                    .ok_or_else(|| format_err!("Invalid setting value handle: {setting_value}"))?;

                let arr = get_arr_mut(memory, value_ptr)?;

                if let settings::Value::I64(value) = setting_value {
                    *arr = value.to_le_bytes();
                    Ok(1u32)
                } else {
                    Ok(0u32)
                }
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "setting_value_get_i64",
        })?
        .func_wrap("env", "setting_value_get_f64", {
            |mut caller: Caller<Context<T>>, setting_value: u64, value_ptr: u32| {
                let (memory, context) = memory_and_context(&mut caller);

                let setting_value = context
                    .setting_values
                    .get(SettingValueKey::from(KeyData::from_ffi(setting_value)))
                    .ok_or_else(|| format_err!("Invalid setting value handle: {setting_value}"))?;

                let arr = get_arr_mut(memory, value_ptr)?;

                if let settings::Value::F64(value) = setting_value {
                    *arr = value.to_le_bytes();
                    Ok(1u32)
                } else {
                    Ok(0u32)
                }
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "setting_value_get_f64",
        })?
        .func_wrap("env", "setting_value_get_string", {
            |mut caller: Caller<Context<T>>, setting_value: u64, buf_ptr: u32, buf_len_ptr: u32| {
                let (memory, context) = memory_and_context(&mut caller);

                let setting_value = context
                    .setting_values
                    .get(SettingValueKey::from(KeyData::from_ffi(setting_value)))
                    .ok_or_else(|| format_err!("Invalid setting value handle: {setting_value}"))?;

                let len_bytes = get_arr_mut(memory, buf_len_ptr)?;

                if let settings::Value::String(value) = setting_value {
                    // Store the original length before updating the pointer.
                    // This ensures the original value is used for error handling logic
                    // to determine if the buffer is large enough to hold the string.
                    let len = u32::from_le_bytes(*len_bytes) as usize;
                    *len_bytes = (value.len() as u32).to_le_bytes();

                    if len < value.len() {
                        return Ok(0u32);
                    }
                    let buf = get_slice_mut(memory, buf_ptr, value.len() as _)?;
                    buf.copy_from_slice(value.as_bytes());
                    Ok(1u32)
                } else {
                    *len_bytes = 0u32.to_le_bytes();
                    Ok(0u32)
                }
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "setting_value_get_string",
        })?;
    Ok(())
}
