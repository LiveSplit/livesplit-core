use crate::{Class, Type, TypeKind};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::io::{Result, Write};

fn get_type(ty: &Type) -> Cow<'_, str> {
    let mut name = Cow::Borrowed(match ty.name.as_str() {
        "i8" => "int8_t",
        "i16" => "int16_t",
        "i32" => "int32_t",
        "i64" => "int64_t",
        "u8" => "uint8_t",
        "u16" => "uint16_t",
        "u32" => "uint32_t",
        "u64" => "uint64_t",
        "usize" => "size_t",
        "f32" => "float",
        "f64" => "double",
        "bool" => "bool",
        "()" => "void",
        "c_char" => "char",
        "Json" => "char const*",
        x => x,
    });
    match (ty.is_custom, ty.kind) {
        (false, TypeKind::RefMut) => name.to_mut().push_str("*restrict"),
        (false, TypeKind::Ref) => name.to_mut().push_str(" const*"),
        (true, TypeKind::RefMut) => name.to_mut().push_str("RefMut"),
        (true, TypeKind::Ref) => name.to_mut().push_str("Ref"),
        _ => (),
    }
    if name == "uint8_t const*" {
        name = Cow::Borrowed("void const*");
    }
    name
}

pub fn write<W: Write>(mut writer: W, classes: &BTreeMap<String, Class>) -> Result<()> {
    write!(
        writer,
        "{}",
        r#"#ifndef LIVESPLIT_CORE_H
#define LIVESPLIT_CORE_H

#ifdef __cplusplus
#define restrict __restrict
namespace LiveSplit {
extern "C" {
#endif

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

"#
    )?;

    for name in classes.keys() {
        writeln!(
            writer,
            r#"struct {0}_s;
typedef struct {0}_s *restrict {0};
typedef struct {0}_s *restrict {0}RefMut;
typedef struct {0}_s const* {0}Ref;
"#,
            name
        )?;
    }

    for class in classes.values() {
        writeln!(writer, "")?;

        for function in class
            .static_fns
            .iter()
            .chain(class.own_fns.iter())
            .chain(class.shared_fns.iter())
            .chain(class.mut_fns.iter())
        {
            if function.method == "drop" {
                writeln!(
                    writer,
                    r#"/**
Frees the object, allowing it to clean up all of its memory. You need
to call this for every object that you don't use anymore and hasn't
already been freed.
*/"#
                )?;
            } else if !function.comments.is_empty() {
                write!(writer, r#"/**"#)?;

                for comment in &function.comments {
                    write!(
                        writer,
                        r#"
{}"#,
                        comment
                            .replace("<NULL>", "NULL")
                            .replace("<TRUE>", "true")
                            .replace("<FALSE>", "false")
                    )?;
                }

                writeln!(
                    writer,
                    r#"
*/"#
                )?;
            }

            write!(
                writer,
                r#"{} {}("#,
                get_type(&function.output),
                function.name
            )?;

            for (i, (name, typ)) in function.inputs.iter().enumerate() {
                if i != 0 {
                    write!(writer, ", ")?;
                }
                write!(
                    writer,
                    "{} {}",
                    get_type(typ),
                    if name == "this" { "self" } else { name }
                )?;
            }
            if function.inputs.is_empty() {
                write!(writer, "void")?;
            }

            writeln!(writer, ");")?;
        }
    }

    write!(
        writer,
        "{}",
        r#"
#ifdef __cplusplus
}
}
#endif

#endif
"#
    )
}
