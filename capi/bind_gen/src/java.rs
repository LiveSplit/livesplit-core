use std::io::{Write, Result};
use {Function, Type, TypeKind};

fn to_camel_case(snake_case: &str) -> String {
    let mut camel_case = String::new();

    for (u, split) in snake_case.split('_').enumerate() {
        for (i, c) in split.char_indices() {
            if u != 0 && i == 0 {
                camel_case.extend(c.to_uppercase());
            } else {
                camel_case.push(c);
            }
        }
    }

    camel_case
}

fn get_type(ty: &Type) -> &str {
    match (ty.kind, ty.name.as_str()) {
        (TypeKind::Ref, "c_char") => "String",
        (TypeKind::Ref, _) |
        (TypeKind::RefMut, _) => "Pointer",
        (_, t) if !ty.is_custom => {
            match t {
                "i8" => "byte",
                "i16" => "short",
                "i32" => "int",
                "i64" => "long",
                "u8" => "byte",
                "u16" => "short",
                "u32" => "int",
                "u64" => "long",
                "usize" => "NativeLong", // Not really correct
                "f32" => "float",
                "f64" => "double",
                "bool" => "byte",
                "()" => "void",
                "c_char" => "byte",
                x => x,
            }
        }
        _ => "Pointer",
    }
}

pub fn write<W: Write>(mut writer: W, functions: &[Function]) -> Result<()> {
    let mut prefix = String::from("");

    write!(writer,
           "{}",
           r#"package livesplitcore;

import com.sun.jna.*;

public interface LiveSplitCore extends Library {
    LiveSplitCore INSTANCE = (LiveSplitCore) Native.loadLibrary("livesplit-core", LiveSplitCore.class);
"#)?;

    for function in functions {
        let name = function.name.to_string();
        let mut splits = name.splitn(2, '_');
        let new_prefix = splits.next().unwrap();
        if !prefix.is_empty() && new_prefix != prefix {
            writeln!(writer, "")?;
        }
        prefix = new_prefix.to_string();

        write!(writer,
               r#"
    {} {}("#,
               get_type(&function.output),
               &function.name)?;

        for (i, &(ref name, ref typ)) in function.inputs.iter().enumerate() {
            if i != 0 {
                write!(writer, ", ")?;
            }
            write!(writer,
                   "{} {}",
                   get_type(typ),
                   if name == "this" {
                       String::from("self")
                   } else {
                       to_camel_case(name)
                   })?;
        }

        write!(writer, ");")?;
    }

    writeln!(writer,
             "{}",
             r#"
}"#)
}
