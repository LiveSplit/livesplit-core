use std::io::{Write, Result};
use Function;
use syntex_syntax::ast::{PathParameters, TyKind};
use syntex_syntax::symbol::Symbol;
use std::collections::BTreeSet;
use std::borrow::Cow;

fn to_camel_case(snake_case: &str) -> String {
    let mut camel_case = String::new();

    for split in snake_case.split('_') {
        for (i, c) in split.char_indices() {
            if i == 0 {
                camel_case.extend(c.to_uppercase());
            } else {
                camel_case.push(c);
            }
        }
    }

    camel_case
}

fn get_type(ty: &TyKind) -> (Cow<'static, str>, bool) {
    if let &TyKind::Ptr(ref ptr) = ty {
        if let &TyKind::Path(_, ref path) = &ptr.ty.node {
            if let Some(segment) = path.segments.last() {
                let name = segment.identifier.name;
                if name == "c_char" {
                    return ("char*".into(), false);
                } else if name == "u8" {
                    return ("void*".into(), false);
                } else if name == "Vec" {
                    if let &PathParameters::AngleBracketed(ref data) =
                        &**segment.parameters.as_ref().unwrap() {
                        if let &TyKind::Path(_, ref path) = &data.types[0].node {
                            if let Some(segment) = path.segments.last() {
                                let name = segment.identifier.name;
                                return (format!("{}List", name).into(), true);
                            }
                        }
                    }
                } else if name == "Component" {
                    return (format!("{}Component",
                                    to_camel_case(&path.segments
                                        .first()
                                        .unwrap()
                                        .identifier
                                        .name
                                        .to_string()))
                                .into(),
                            true);
                } else {
                    return (name.to_string().into(), true);
                }
            }
        }
    } else if let &TyKind::Path(_, ref path) = ty {
        if let Some(segment) = path.segments.last() {
            let name = segment.identifier.name;
            if name == "i8" {
                return ("int8_t".into(), false);
            } else if name == "i16" {
                return ("int16_t".into(), false);
            } else if name == "i32" {
                return ("int32_t".into(), false);
            } else if name == "i64" {
                return ("int64_t".into(), false);
            } else if name == "u8" {
                return ("uint8_t".into(), false);
            } else if name == "u16" {
                return ("uint16_t".into(), false);
            } else if name == "u32" {
                return ("uint32_t".into(), false);
            } else if name == "u64" {
                return ("uint64_t".into(), false);
            } else if name == "usize" {
                return ("size_t".into(), false);
            } else if name == "f32" {
                return ("float".into(), false);
            } else if name == "f64" {
                return ("double".into(), false);
            } else if name == "bool" {
                return ("_Bool".into(), false);
            } else if name == "TimingMethod" {
                return ("uint8_t".into(), false);
            } else if name == "TimerPhase" {
                return ("uint8_t".into(), false);
            }
        }
    }
    panic!("Unknown type {:#?}", ty);
}

pub fn write<W: Write>(mut writer: W, functions: &[Function]) -> Result<()> {
    let mut prefix = String::from("");

    write!(writer,
           "{}",
           r#"#ifndef _LIVESPLIT_CORE_H_
#define _LIVESPLIT_CORE_H_

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

"#)?;

    let mut types = BTreeSet::new();

    for function in functions {
        if let Some(output) = function.output {
            if let (custom_type, true) = get_type(output) {
                types.insert(custom_type);
            }
        }
        for &(_, ref typ) in &function.inputs {
            if let (custom_type, true) = get_type(typ) {
                types.insert(custom_type);
            }
        }
    }

    for custom_type in types {
        writeln!(writer,
                 r#"struct {0}_s;
typedef struct {0}_s* {0};
"#,
                 custom_type)?;
    }

    for function in functions {
        let name = function.name.to_string();
        let new_prefix = name.split('_').next().unwrap();
        if !prefix.is_empty() && new_prefix != prefix {
            writeln!(writer, "")?;
        }
        prefix = new_prefix.to_string();

        write!(writer,
               r#"extern {} {}("#,
               function.output
                   .map(get_type)
                   .map(|(t, _)| t)
                   .unwrap_or("void".into()),
               function.name)?;

        for (i, &(name, typ)) in function.inputs.iter().enumerate() {
            if i != 0 {
                write!(writer, ", ")?;
            }
            write!(writer,
                   "{} {}",
                   get_type(typ).0,
                   name.map(|n| if n == "this" {
                           Symbol::intern("self")
                       } else if n == "this_drop" {
                           Symbol::intern("self_drop")
                       } else {
                           n
                       })
                       .unwrap_or_else(|| Symbol::intern("parameter")))?;
        }
        if function.inputs.is_empty() {
            write!(writer, "void")?;
        }

        writeln!(writer, ");")?;
    }

    write!(writer,
           "{}",
           r#"
#endif
"#)
}
