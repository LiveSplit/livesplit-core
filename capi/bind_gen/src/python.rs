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
        (TypeKind::Ref, "c_char") => "c_char_p",
        (TypeKind::Ref, _) |
        (TypeKind::RefMut, _) => "c_void_p",
        (_, t) if !ty.is_custom => {
            match t {
                "i8" => "c_int8",
                "i16" => "c_int16",
                "i32" => "c_int32",
                "i64" => "c_int64",
                "u8" => "c_uint8",
                "u16" => "c_uint16",
                "u32" => "c_uint32",
                "u64" => "c_uint64",
                "usize" => "c_size_t",
                "f32" => "c_float",
                "f64" => "c_double",
                "bool" => "c_bool",
                "()" => "None",
                "c_char" => "c_char",
                x => x,
            }
        }
        _ => "c_void_p",
    }
}

pub fn write<W: Write>(mut writer: W, functions: &[Function]) -> Result<()> {
    let mut prefix = String::from("");

    write!(writer,
           "{}",
           r#"#!/usr/bin/env python3
# coding: utf-8

import sys, ctypes
from ctypes import c_char_p, c_void_p, c_int8, c_int16, c_int32, c_int64, c_uint8, c_uint16, c_uint32, c_uint64, c_size_t, c_float, c_double, c_bool, c_char, c_byte

prefix = {'win32': ''}.get(sys.platform, 'lib')
extension = {'darwin': '.dylib', 'win32': '.dll'}.get(sys.platform, '.so')
lib = ctypes.cdll.LoadLibrary(prefix + "livesplit_core" + extension)
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
lib.{}.argtypes = ("#,
               function.name)?;

        for &(_, ref typ) in &function.inputs {
            write!(writer, "{}, ", get_type(typ))?;
        }

        write!(writer,
               r#")
lib.{}.restype = {}"#,
               function.name,
               get_type(&function.output))?;
    }

    writeln!(writer)?;

    let mut prefix = String::from("");

    for function in functions {
        let name = function.name.to_string();
        let mut splits = name.splitn(2, '_');
        let new_prefix = splits.next().unwrap();
        let postfix = splits.next().unwrap();
        if !prefix.is_empty() && new_prefix != prefix {
            writeln!(writer, "")?;
        }
        prefix = new_prefix.to_string();

        write!(writer,
               r#"
def {}_{}("#,
               prefix,
               to_camel_case(postfix))?;

        for (i, &(ref name, _)) in function.inputs.iter().enumerate() {
            if i != 0 {
                write!(writer, ", ")?;
            }
            write!(writer, "{}", to_camel_case(name))?;
        }

        write!(writer,
               r#"):
    "#)?;

        if get_type(&function.output) != "None" {
            write!(writer, "return ")?;
        }

        write!(writer, "lib.{}(", function.name)?;

        for (i, &(ref name, _)) in function.inputs.iter().enumerate() {
            if i != 0 {
                write!(writer, ", ")?;
            }
            write!(writer, "{}", to_camel_case(name))?;
        }

        write!(writer,
               r#")
"#)?;
    }

    writeln!(writer,
             r#"
def Run_parseFile(file):
    bytes = bytearray(file.read())
    bufferType = c_byte * len(bytes)
    buffer = bufferType(*bytes)
    return Run_parse(buffer, len(bytes))"#)
}
