use std::io::{Write, Result};
use {Function, Type, TypeKind};
use std::collections::BTreeSet;
use std::borrow::Cow;

fn get_type(ty: &Type) -> Cow<str> {
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
        "bool" => "uint8_t",
        "()" => "void",
        "c_char" => "char",
        x => x,
    });
    match (ty.is_custom, ty.kind) {
        (false, TypeKind::RefMut) => name.to_mut().push_str("*"),
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

pub fn write<W: Write>(mut writer: W, functions: &[Function]) -> Result<()> {
    let mut prefix = String::from("");

    write!(writer,
           "{}",
           r#"#ifndef _LIVESPLIT_CORE_H_
#define _LIVESPLIT_CORE_H_

#ifdef __cplusplus
namespace LiveSplit {
extern "C" {
#endif

#include <stdint.h>
#include <stddef.h>

"#)?;

    let mut types = BTreeSet::new();

    for function in functions {
        let output = &function.output;
        if output.is_custom {
            types.insert(&output.name);
        }

        for &(_, ref typ) in &function.inputs {
            if typ.is_custom {
                types.insert(&typ.name);
            }
        }
    }

    for custom_type in types {
        writeln!(writer,
                 r#"struct {0}_s;
typedef struct {0}_s* {0};
typedef struct {0}_s* {0}RefMut;
typedef struct {0}_s const* {0}Ref;
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
               get_type(&function.output),
               function.name)?;

        for (i, &(ref name, ref typ)) in function.inputs.iter().enumerate() {
            if i != 0 {
                write!(writer, ", ")?;
            }
            write!(writer,
                   "{} {}",
                   get_type(typ),
                   if name == "this" { "self" } else { name })?;
        }
        if function.inputs.is_empty() {
            write!(writer, "void")?;
        }

        writeln!(writer, ");")?;
    }

    write!(writer,
           "{}",
           r#"
#ifdef __cplusplus
}
}
#endif

#endif
"#)
}
