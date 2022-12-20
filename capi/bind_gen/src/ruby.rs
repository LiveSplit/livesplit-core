use crate::{Class, Function, Opt, Type, TypeKind};
use std::{
    collections::BTreeMap,
    io::{Result, Write},
};

fn get_hl_type(ty: &Type) -> String {
    if ty.is_custom {
        let name = ty.name.to_string();
        match ty.kind {
            TypeKind::Ref => format!("{}Ref", name),
            TypeKind::RefMut => format!("{}RefMut", name),
            TypeKind::Value => name,
        }
    } else {
        match (ty.kind, ty.name.as_str()) {
            (TypeKind::Ref, "c_char") => "String",
            (_, t) if !ty.is_custom => match t {
                "i8" => "Integer",
                "i16" => "Integer",
                "i32" => "Integer",
                "i64" => "Integer",
                "u8" => "Integer",
                "u16" => "Integer",
                "u32" => "Integer",
                "u64" => "Integer",
                "usize" => "Integer",
                "isize" => "Integer",
                "f32" => "Float",
                "f64" => "Float",
                "bool" => "Boolean",
                "()" => "void",
                "c_char" => "String",
                "Json" => "String",
                x => x,
            },
            _ => "Object",
        }
        .to_string()
    }
}

fn get_ll_type(ty: &Type) -> &str {
    match (ty.kind, ty.name.as_str()) {
        (TypeKind::Ref, "c_char") => "string",
        (TypeKind::Ref, _) | (TypeKind::RefMut, _) => "pointer",
        (_, t) if !ty.is_custom => match t {
            "i8" => "int8",
            "i16" => "int16",
            "i32" => "int32",
            "i64" => "int64",
            "u8" => "uint8",
            "u16" => "uint16",
            "u32" => "uint32",
            "u64" => "uint64",
            "usize" => "size_t",
            "isize" => "ssize_t",
            "f32" => "float",
            "f64" => "double",
            "bool" => "bool",
            "()" => "void",
            "c_char" => "char",
            "Json" => "string",
            x => x,
        },
        _ => "pointer",
    }
}

fn ptr_of(var: &str) -> String {
    if var == "this" {
        "@handle.ptr".to_string()
    } else {
        format!("{}.handle.ptr", var)
    }
}

fn write_class_comments<W: Write>(mut writer: W, comments: &[String]) -> Result<()> {
    writeln!(writer)?;

    for comment in comments {
        write!(
            writer,
            r#"
    # {}"#,
            comment
                .replace("<NULL>", "nil")
                .replace("<TRUE>", "true")
                .replace("<FALSE>", "false")
        )?;
    }

    Ok(())
}

fn write_fn<W: Write>(mut writer: W, function: &Function) -> Result<()> {
    let is_static = function.is_static();
    let has_return_type = function.has_return_type();
    let return_type = get_hl_type(&function.output);
    let mut method: &str = &function.method;
    if method == "new" {
        method = "create";
    }

    for comment in &function.comments {
        write!(
            writer,
            r#"
        # {}"#,
            comment
                .replace("<NULL>", "nil")
                .replace("<TRUE>", "true")
                .replace("<FALSE>", "false")
        )?;
    }

    for (name, typ) in function.inputs.iter().skip(usize::from(!is_static)) {
        write!(
            writer,
            r"
        # @param [{}{}] {}",
            get_hl_type(typ),
            if typ.is_nullable { ", nil" } else { "" },
            name
        )?;
    }

    if has_return_type {
        write!(
            writer,
            r"
        # @return [{}{}]",
            return_type,
            if function.output.is_nullable {
                ", nil"
            } else {
                ""
            }
        )?;
    }

    write!(
        writer,
        r#"
        def {}{}("#,
        if is_static { "self." } else { "" },
        method
    )?;

    for (i, (name, _)) in function
        .inputs
        .iter()
        .skip(usize::from(!is_static))
        .enumerate()
    {
        if i != 0 {
            write!(writer, ", ")?;
        }
        write!(writer, "{}", name)?;
    }

    write!(
        writer,
        r#")
            "#
    )?;

    for (name, typ) in function.inputs.iter() {
        if typ.is_custom {
            write!(
                writer,
                r#"if {ptr} == nil
                raise "{name} is disposed"
            end
            "#,
                name = name,
                ptr = ptr_of(name)
            )?;
        }
    }

    if has_return_type {
        if function.output.is_custom {
            write!(writer, r#"result = {}.new("#, return_type)?;
        } else {
            write!(writer, "result = ")?;
        }
    }

    write!(writer, r#"Native.{}("#, &function.name)?;

    for (i, (name, typ)) in function.inputs.iter().enumerate() {
        if i != 0 {
            write!(writer, ", ")?;
        }
        write!(
            writer,
            "{}",
            if name == "this" {
                "@handle.ptr".to_string()
            } else if typ.is_custom {
                format!("{}.handle.ptr", name)
            } else {
                name.to_string()
            }
        )?;
    }

    write!(writer, ")")?;

    if has_return_type && function.output.is_custom {
        write!(writer, r#")"#)?;
    }

    for (name, typ) in function.inputs.iter() {
        if typ.is_custom && typ.kind == TypeKind::Value {
            write!(
                writer,
                r#"
            {} = nil"#,
                ptr_of(name)
            )?;
        }
    }

    if has_return_type {
        if function.output.is_nullable && function.output.is_custom {
            write!(
                writer,
                r#"
            if result.handle.ptr == nil
                return nil
            end"#
            )?;
        }
        write!(
            writer,
            r#"
            result"#
        )?;
    }

    write!(
        writer,
        r#"
        end"#
    )?;

    Ok(())
}

pub fn write<W: Write>(mut writer: W, classes: &BTreeMap<String, Class>, opt: &Opt) -> Result<()> {
    write!(
        writer,
        r#"# coding: utf-8
require 'ffi'

module LiveSplitCore
    module Native
        extend FFI::Library
        ffi_lib File.expand_path('{}', __FILE__)
    "#,
        opt.ruby_lib_path
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
        attach_function :{}, ["#,
                function.name
            )?;

            for (i, (_, typ)) in function.inputs.iter().enumerate() {
                if i != 0 {
                    write!(writer, ", ")?;
                }
                write!(writer, ":{}", get_ll_type(typ))?;
            }

            write!(writer, "], :{}", get_ll_type(&function.output))?;
        }
    }

    write!(
        writer,
        "{}",
        r#"
    end

    class LSCHandle
        attr_accessor :ptr
        def initialize(ptr)
            @ptr = ptr
        end
    end"#
    )?;

    for (class_name, class) in classes {
        let class_name = class_name.to_string();
        let class_name_ref = format!("{}Ref", class_name);
        let class_name_ref_mut = format!("{}RefMut", class_name);

        write_class_comments(&mut writer, &class.comments)?;

        write!(
            writer,
            r#"
    class {class}
        attr_accessor :handle"#,
            class = class_name_ref
        )?;

        for function in &class.shared_fns {
            write_fn(&mut writer, function)?;
        }

        if class_name == "SharedTimer" {
            write!(
                writer,
                "{}",
                r#"
        def read_with
            self.read.wtih do |lock|
                yield lock.timer
            end
        end
        def write_with
            self.write.with do |lock|
                yield lock.timer
            end
        end"#
            )?;
        }

        write!(
            writer,
            r#"
        def initialize(ptr)
            @handle = LSCHandle.new ptr
        end
    end"#
        )?;

        write_class_comments(&mut writer, &class.comments)?;

        write!(
            writer,
            r#"
    class {class} < {base_class}"#,
            class = class_name_ref_mut,
            base_class = class_name_ref
        )?;

        for function in &class.mut_fns {
            write_fn(&mut writer, function)?;
        }

        write!(
            writer,
            r#"
        def initialize(ptr)
            @handle = LSCHandle.new ptr
        end
    end"#,
        )?;

        write_class_comments(&mut writer, &class.comments)?;

        write!(
            writer,
            r#"
    class {class} < {base_class}
        def self.finalize(handle)
            proc {{
                if handle.ptr != nil"#,
            class = class_name,
            base_class = class_name_ref_mut
        )?;

        if let Some(function) = class.own_fns.iter().find(|f| f.method == "drop") {
            write!(
                writer,
                r#"
                    Native.{} handle.ptr"#,
                function.name
            )?;
        }

        write!(
            writer,
            r#"
                    handle.ptr = nil
                end
            }}
        end
        def dispose
            finalizer = {class}.finalize @handle
            finalizer.call
        end
        def with
            yield self
            self.dispose
        end"#,
            class = class_name
        )?;

        for function in class.static_fns.iter().chain(class.own_fns.iter()) {
            if function.method != "drop" {
                write_fn(&mut writer, function)?;
            }
        }

        write!(
            writer,
            r#"
        def initialize(ptr)
            handle = LSCHandle.new ptr
            @handle = handle
            ObjectSpace.define_finalizer(self, self.class.finalize(handle))
        end
    end"#
        )?;
    }

    writeln!(
        writer,
        r#"
end"#
    )
}
