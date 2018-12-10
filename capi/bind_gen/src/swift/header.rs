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
        (false, TypeKind::RefMut) => name.to_mut().push_str("*"),
        (false, TypeKind::Ref) => {
            if name == "char" {
                name.to_mut().push_str(" const*")
            } else {
                name.to_mut().push_str("*")
            }
        }
        (true, TypeKind::Ref) => name = Cow::Borrowed("void*"),
        (true, _) => name = Cow::Borrowed("void*"),
        _ => (),
    }
    if name == "uint8_t*" {
        name = Cow::Borrowed("void*");
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
namespace LiveSplit {
extern "C" {
#endif

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
"#
    )?;

    for class in classes.values() {
        writeln!(writer, "")?;

        for function in class
            .static_fns
            .iter()
            .chain(class.own_fns.iter())
            .chain(class.shared_fns.iter())
            .chain(class.mut_fns.iter())
        {
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
