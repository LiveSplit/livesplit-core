use std::sync::Arc;

use anyhow::{Context as _, Result, bail};
use wasmtime::{Caller, Linker};

use crate::{CreationError, Timer, runtime::Context, settings};

use super::{get_str, memory_and_context};

pub fn bind<T: Timer>(linker: &mut Linker<Context<T>>) -> Result<(), CreationError> {
    linker
        .func_wrap("env", "user_settings_add_bool", {
            |mut caller: Caller<Context<T>>,
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
            |mut caller: Caller<Context<T>>,
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
            |mut caller: Caller<Context<T>>,
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
            |mut caller: Caller<Context<T>>,
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
        .func_wrap("env", "user_settings_add_file_select", {
            |mut caller: Caller<Context<T>>,
             key_ptr: u32,
             key_len: u32,
             description_ptr: u32,
             description_len: u32| {
                let (memory, context) = memory_and_context(&mut caller);
                let key = get_str(memory, key_ptr, key_len)?.into();
                let description = get_str(memory, description_ptr, description_len)?.into();
                Arc::make_mut(&mut context.settings_widgets).push(settings::Widget {
                    key,
                    description,
                    tooltip: None,
                    kind: settings::WidgetKind::FileSelect {
                        filters: Arc::new(Vec::new()),
                    },
                });
                Ok(())
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "user_settings_add_file_select",
        })?
        .func_wrap("env", "user_settings_add_file_select_name_filter", {
            |mut caller: Caller<Context<T>>,
             key_ptr: u32,
             key_len: u32,
             description_ptr: u32,
             description_len: u32,
             pattern_ptr: u32,
             pattern_len: u32| {
                let (memory, context) = memory_and_context(&mut caller);
                let key = get_str(memory, key_ptr, key_len)?.into();
                let description = if description_ptr != 0 {
                    Some(get_str(memory, description_ptr, description_len)?.into())
                } else {
                    None
                };
                let pattern = get_str(memory, pattern_ptr, pattern_len)?.into();
                let setting = Arc::make_mut(&mut context.settings_widgets)
                    .iter_mut()
                    .find(|s| s.key == key)
                    .context("There is no setting with the provided key.")?;
                let settings::WidgetKind::FileSelect { filters } = &mut setting.kind else {
                    bail!("The setting is not a file select.");
                };
                Arc::make_mut(filters).push(settings::FileFilter::Name {
                    description,
                    pattern,
                });
                Ok(())
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "user_settings_add_file_select_name_filter",
        })?
        .func_wrap("env", "user_settings_add_file_select_mime_filter", {
            |mut caller: Caller<Context<T>>,
             key_ptr: u32,
             key_len: u32,
             mime_ptr: u32,
             mime_len: u32| {
                let (memory, context) = memory_and_context(&mut caller);
                let key = get_str(memory, key_ptr, key_len)?.into();
                let mime = get_str(memory, mime_ptr, mime_len)?.into();
                let setting = Arc::make_mut(&mut context.settings_widgets)
                    .iter_mut()
                    .find(|s| s.key == key)
                    .context("There is no setting with the provided key.")?;
                let settings::WidgetKind::FileSelect { filters } = &mut setting.kind else {
                    bail!("The setting is not a file select.");
                };
                Arc::make_mut(filters).push(settings::FileFilter::MimeType(mime));
                Ok(())
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "user_settings_add_file_select_mime_filter",
        })?
        .func_wrap("env", "user_settings_set_tooltip", {
            |mut caller: Caller<Context<T>>,
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
