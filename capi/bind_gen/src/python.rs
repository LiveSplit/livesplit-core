use crate::{Class, Function, Type, TypeKind};
use std::{
    collections::BTreeMap,
    io::{Result, Write},
};

fn get_hl_type(ty: &Type) -> String {
    if ty.is_custom {
        match ty.kind {
            TypeKind::Ref => format!("{}Ref", ty.name),
            TypeKind::RefMut => format!("{}RefMut", ty.name),
            TypeKind::Value => ty.name.clone(),
        }
    } else {
        get_ll_type(ty).to_string()
    }
}

fn get_ll_type(ty: &Type) -> &str {
    match (ty.kind, ty.name.as_str()) {
        (TypeKind::Ref, "c_char") => "c_char_p",
        (TypeKind::Ref, _) | (TypeKind::RefMut, _) => "c_void_p",
        (_, t) if !ty.is_custom => match t {
            "i8" => "c_int8",
            "i16" => "c_int16",
            "i32" => "c_int32",
            "i64" => "c_int64",
            "u8" => "c_uint8",
            "u16" => "c_uint16",
            "u32" => "c_uint32",
            "u64" => "c_uint64",
            "usize" => "c_size_t",
            "isize" => "c_ssize_t",
            "f32" => "c_float",
            "f64" => "c_double",
            "bool" => "c_bool",
            "()" => "None",
            "c_char" => "c_char",
            "Json" => "c_char_p",
            x => x,
        },
        _ => "c_void_p",
    }
}

fn map_var(var: &str) -> &str {
    if var == "this" {
        "self"
    } else {
        var
    }
}

fn write_class_comments<W: Write>(mut writer: W, comments: &[String]) -> Result<()> {
    write!(
        writer,
        r#"
    """"#
    )?;

    for comment in comments {
        write!(
            writer,
            r#"{}
    "#,
            comment
                .replace("<NULL>", "None")
                .replace("<TRUE>", "True")
                .replace("<FALSE>", "False")
        )?;
    }

    writeln!(writer, r#"""""#)
}

fn write_fn<W: Write>(mut writer: W, function: &Function) -> Result<()> {
    let is_static = function.is_static();
    let has_return_type = function.has_return_type();
    let return_type = get_hl_type(&function.output);

    if is_static {
        write!(
            writer,
            r#"
    @staticmethod
    def {}("#,
            function.method
        )?;
    } else {
        write!(
            writer,
            r#"
    def {}("#,
            function.method
        )?;
    }

    for (i, (name, _)) in function.inputs.iter().enumerate() {
        if i != 0 {
            write!(writer, ", ")?;
        }
        write!(writer, "{}", map_var(name))?;
    }

    write!(
        writer,
        r#"):
        "#
    )?;

    write!(writer, r#"""""#)?;

    for comment in &function.comments {
        write!(
            writer,
            r#"{}
        "#,
            comment
                .replace("<NULL>", "None")
                .replace("<TRUE>", "True")
                .replace("<FALSE>", "False")
        )?;
    }

    write!(
        writer,
        r#""""
        "#
    )?;

    for (name, typ) in function.inputs.iter() {
        if typ.is_custom {
            write!(
                writer,
                r#"if {name}.ptr == None:
            raise Exception("{name} is disposed")
        "#,
                name = map_var(name)
            )?;
        }
    }

    if has_return_type {
        if function.output.is_custom {
            write!(writer, r#"result = {}("#, return_type)?;
        } else {
            write!(writer, "result = ")?;
        }
    }

    write!(writer, r#"livesplit_core_native.{}("#, &function.name)?;

    for (i, (name, typ)) in function.inputs.iter().enumerate() {
        if i != 0 {
            write!(writer, ", ")?;
        }
        write!(
            writer,
            "{}",
            if name == "this" {
                "self.ptr".to_string()
            } else if typ.is_custom {
                format!("{}.ptr", name)
            } else if typ.name == "c_char" {
                format!("{}.encode()", name)
            } else {
                name.to_string()
            }
        )?;
    }

    write!(writer, ")")?;

    if has_return_type {
        if function.output.is_custom {
            write!(writer, r#")"#)?;
        } else if function.output.name == "c_char" {
            write!(writer, r#".decode()"#)?;
        }
    }

    for (name, typ) in function.inputs.iter() {
        if typ.is_custom && typ.kind == TypeKind::Value {
            write!(
                writer,
                r#"
        {}.ptr = None"#,
                map_var(name)
            )?;
        }
    }

    if has_return_type {
        if function.output.is_nullable && function.output.is_custom {
            write!(
                writer,
                r#"
        if result.ptr == None:
            return None"#
            )?;
        }
        write!(
            writer,
            r#"
        return result"#
        )?;
    }

    writeln!(writer)?;

    Ok(())
}

pub fn write<W: Write>(mut writer: W, classes: &BTreeMap<String, Class>) -> Result<()> {
    write!(
        writer,
        "{}",
        r#"#!/usr/bin/env python3
# coding: utf-8

import sys, ctypes
from ctypes import c_char_p, c_void_p, c_int8, c_int16, c_int32, c_int64, c_uint8, c_uint16, c_uint32, c_uint64, c_size_t, c_ssize_t, c_float, c_double, c_bool, c_char, c_byte

prefix = {'win32': ''}.get(sys.platform, './lib')
extension = {'darwin': '.dylib', 'win32': '.dll'}.get(sys.platform, '.so')
livesplit_core_native = ctypes.cdll.LoadLibrary(prefix + "livesplit_core" + extension)
"#
    )?;

    for class in classes.values() {
        for function in class
            .static_fns
            .iter()
            .chain(class.own_fns.iter())
            .chain(class.shared_fns.iter())
            .chain(class.mut_fns.iter())
        {
            write!(
                writer,
                r#"
livesplit_core_native.{}.argtypes = ("#,
                function.name
            )?;

            for (_, typ) in &function.inputs {
                write!(writer, "{}, ", get_ll_type(typ))?;
            }

            write!(
                writer,
                r#")
livesplit_core_native.{}.restype = {}"#,
                function.name,
                get_ll_type(&function.output)
            )?;
        }
    }

    writeln!(writer)?;

    for (class_name, class) in classes {
        let class_name_ref = format!("{}Ref", class_name);
        let class_name_ref_mut = format!("{}RefMut", class_name);

        write!(
            writer,
            r#"
class {class}:"#,
            class = class_name_ref
        )?;

        write_class_comments(&mut writer, &class.comments)?;

        for function in &class.shared_fns {
            write_fn(&mut writer, function)?;
        }

        write!(
            writer,
            r#"
    def __init__(self, ptr):
        self.ptr = ptr

class {class}({base_class}):"#,
            class = class_name_ref_mut,
            base_class = class_name_ref
        )?;

        write_class_comments(&mut writer, &class.comments)?;

        for function in &class.mut_fns {
            write_fn(&mut writer, function)?;
        }

        write!(
            writer,
            r#"
    def __init__(self, ptr):
        self.ptr = ptr

class {class}({base_class}):"#,
            class = class_name,
            base_class = class_name_ref_mut
        )?;

        write_class_comments(&mut writer, &class.comments)?;

        write!(
            writer,
            r#"
    def drop(self):
        if self.ptr != None:"#,
        )?;

        if let Some(function) = class.own_fns.iter().find(|f| f.method == "drop") {
            write!(
                writer,
                r#"
            livesplit_core_native.{}(self.ptr)"#,
                function.name
            )?;
        }

        write!(
            writer,
            r#"
            self.ptr = None

    def __del__(self):
        self.drop()

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.drop()
"#
        )?;

        for function in class.static_fns.iter().chain(class.own_fns.iter()) {
            if function.method != "drop" {
                write_fn(&mut writer, function)?;
            }
        }

        if class_name == "Run" {
            writeln!(
                writer,
                r#"
    @staticmethod
    def parse_file(file, load_files_path):
        data = file.read()
        if sys.version_info[0] > 2:
            if isinstance(data, str):
                raise TypeError("File must be opened in binary mode!")
        bytes = bytearray(data)
        bufferType = c_byte * len(bytes)
        buffer = bufferType(*bytes)
        return Run.parse(buffer, len(bytes), load_files_path)"#
            )?;
        }

        write!(
            writer,
            r#"
    def __init__(self, ptr):
        self.ptr = ptr
"#
        )?;
    }

    Ok(())
}
