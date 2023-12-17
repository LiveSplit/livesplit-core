use std::sync::Arc;

use anyhow::{bail, Context as _, Result};
use wasmtime::{Caller, Linker};

use crate::{runtime::Context, settings, CreationError, Timer};

use super::{get_str, memory_and_context};

pub fn bind<T: Timer>(linker: &mut Linker<Context<T>>) -> Result<(), CreationError> {
    linker
        .func_wrap("env", "user_settings_add_bool", {
            |mut caller: Caller<'_, Context<T>>,
             key_ptr: u32,
             key_len: u32,
             description_ptr: u32,
             description_len: u32,
             default_value: u32| {
                let (memory, context) = memory_and_context(&mut caller);
                let key = Arc::<str>::from(get_str(memory, key_ptr, key_len)?);
                let description = get_str(memory, description_ptr, description_len)?.into();
                let default_value = default_value != 0;
                let value_in_map = match context.shared_data.get_settings_map().get(&key) {
                    Some(settings::Value::Bool(v)) => *v,
                    _ => default_value,
                };
                Arc::make_mut(&mut context.settings_widgets).push(settings::Widget {
                    key,
                    description,
                    tooltip: None,
                    kind: settings::WidgetKind::Bool { default_value },
                });
                Ok(value_in_map as u32)
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "user_settings_add_bool",
        })?
        .func_wrap("env", "user_settings_add_title", {
            |mut caller: Caller<'_, Context<T>>,
             key_ptr: u32,
             key_len: u32,
             description_ptr: u32,
             description_len: u32,
             heading_level: u32| {
                let (memory, context) = memory_and_context(&mut caller);
                let key = get_str(memory, key_ptr, key_len)?.into();
                let description = get_str(memory, description_ptr, description_len)?.into();
                Arc::make_mut(&mut context.settings_widgets).push(settings::Widget {
                    key,
                    description,
                    tooltip: None,
                    kind: settings::WidgetKind::Title { heading_level },
                });
                Ok(())
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "user_settings_add_title",
        })?
        .func_wrap("env", "user_settings_add_choice", {
            |mut caller: Caller<'_, Context<T>>,
             key_ptr: u32,
             key_len: u32,
             description_ptr: u32,
             description_len: u32,
             default_option_key_ptr: u32,
             default_option_key_len: u32| {
                let (memory, context) = memory_and_context(&mut caller);
                let key = get_str(memory, key_ptr, key_len)?.into();
                let description = get_str(memory, description_ptr, description_len)?.into();
                let default_option_key =
                    get_str(memory, default_option_key_ptr, default_option_key_len)?.into();
                Arc::make_mut(&mut context.settings_widgets).push(settings::Widget {
                    key,
                    description,
                    tooltip: None,
                    kind: settings::WidgetKind::Choice {
                        default_option_key,
                        options: Arc::new(Vec::new()),
                    },
                });
                Ok(())
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "user_settings_add_choice",
        })?
        .func_wrap("env", "user_settings_add_choice_option", {
            |mut caller: Caller<'_, Context<T>>,
             key_ptr: u32,
             key_len: u32,
             option_key_ptr: u32,
             option_key_len: u32,
             option_description_ptr: u32,
             option_description_len: u32| {
                let (memory, context) = memory_and_context(&mut caller);
                let key = get_str(memory, key_ptr, key_len)?.into();
                let option_key = get_str(memory, option_key_ptr, option_key_len)?.into();
                let option_description =
                    get_str(memory, option_description_ptr, option_description_len)?.into();
                let setting = Arc::make_mut(&mut context.settings_widgets)
                    .iter_mut()
                    .find(|s| s.key == key)
                    .context("There is no setting with the provided key.")?;
                let (options, is_chosen) = match &mut setting.kind {
                    settings::WidgetKind::Choice {
                        options,
                        default_option_key,
                    } => (options, *default_option_key == option_key),
                    _ => bail!("The setting is not a choice."),
                };
                Arc::make_mut(options).push(settings::ChoiceOption {
                    key: option_key,
                    description: option_description,
                });
                Ok(is_chosen as u32)
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "user_settings_add_choice_option",
        })?
        .func_wrap("env", "user_settings_set_tooltip", {
            |mut caller: Caller<'_, Context<T>>,
             key_ptr: u32,
             key_len: u32,
             tooltip_ptr: u32,
             tooltip_len: u32| {
                let (memory, context) = memory_and_context(&mut caller);
                let key = get_str(memory, key_ptr, key_len)?.into();
                let tooltip = get_str(memory, tooltip_ptr, tooltip_len)?.into();
                Arc::make_mut(&mut context.settings_widgets)
                    .iter_mut()
                    .find(|s| s.key == key)
                    .context("There is no setting with the provided key.")?
                    .tooltip = Some(tooltip);
                Ok(())
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "user_settings_set_tooltip",
        })?;
    Ok(())
}
