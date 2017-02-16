use std::io::{Write, Result};
use {Function, Type, TypeKind};

fn get_type(ty: &Type) -> &str {
    match (ty.kind, ty.name.as_str()) {
        (TypeKind::Ref, "c_char") => "string",
        (TypeKind::Ref, _) |
        (TypeKind::RefMut, _) => "pointer",
        (_, t) if !ty.is_custom => {
            match t {
                "i8" => "int8",
                "i16" => "int16",
                "i32" => "int32",
                "i64" => "int64",
                "u8" => "uint8",
                "u16" => "uint16",
                "u32" => "uint32",
                "u64" => "uint64",
                "usize" => "size_t",
                "f32" => "float",
                "f64" => "double",
                "bool" => "bool",
                "()" => "void",
                "c_char" => "char",
                x => x,
            }
        }
        _ => "pointer",
    }
}

pub fn write<W: Write>(mut writer: W, functions: &[Function]) -> Result<()> {
    let mut prefix = String::from("");

    write!(writer,
           "{}",
           r#"# coding: utf-8
require 'ffi'

module LiveSplitCore
    extend FFI::Library
    ffi_lib './liblivesplit-core.so'
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
    attach_function :{}, ["#,
               function.name)?;

        for (i, &(_, ref typ)) in function.inputs.iter().enumerate() {
            if i != 0 {
                write!(writer, ", ")?;
            }
            write!(writer, ":{}", get_type(typ))?;
        }

        write!(writer, "], :{}", get_type(&function.output))?;
    }

    writeln!(writer,
             "{}",
             r#"
end"#)
}
