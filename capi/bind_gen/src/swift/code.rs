use crate::{Class, Function, Type, TypeKind};
use heck::MixedCase;
use std::collections::BTreeMap;
use std::io::{Result, Write};

fn get_hl_type(ty: &Type) -> String {
    if ty.is_custom {
        match ty.kind {
            TypeKind::Ref => format!("{}Ref", ty.name),
            TypeKind::RefMut => format!("{}RefMut", ty.name),
            TypeKind::Value => ty.name.clone(),
        }
    } else if ty.name == "bool" {
        "Bool".to_string()
    } else if ty.name == "usize" {
        "size_t".to_string()
    } else {
        get_ll_type(ty).to_string()
    }
}

fn get_ll_type(ty: &Type) -> &str {
    match (ty.kind, ty.name.as_str()) {
        (TypeKind::Ref, "c_char") => "String",
        (TypeKind::Ref, _) | (TypeKind::RefMut, _) => "UnsafeMutableRawPointer?",
        (_, t) if !ty.is_custom => match t {
            "i8" => "Int8",
            "i16" => "Int16",
            "i32" => "Int32",
            "i64" => "Int64",
            "u8" => "UInt8",
            "u16" => "UInt16",
            "u32" => "UInt32",
            "u64" => "UInt64",
            "usize" => "size_t",
            "f32" => "Float",
            "f64" => "Double",
            "bool" => "Bool",
            "()" => "()",
            "c_char" => "UInt8",
            "Json" => "String",
            x => x,
        },
        _ => "UnsafeMutableRawPointer?",
    }
}

fn write_class_comments<W: Write>(mut writer: W, comments: &[String]) -> Result<()> {
    write!(
        writer,
        r#"
/**"#
    )?;

    for comment in comments {
        write!(
            writer,
            r#"
    {}"#,
            comment
                .replace("<NULL>", "nil")
                .replace("<TRUE>", "true")
                .replace("<FALSE>", "false")
        )?;
    }

    write!(
        writer,
        r#"
*/"#
    )
}

fn write_fn<W: Write>(mut writer: W, function: &Function) -> Result<()> {
    let is_static = function.is_static();
    let has_return_type = function.has_return_type();
    let return_type = get_hl_type(&function.output);
    let is_constructor = function.method == "new";

    if !function.comments.is_empty() {
        write!(
            writer,
            r#"
    /**"#
        )?;

        for comment in &function.comments {
            write!(
                writer,
                r#"
        {}"#,
                comment
                    .replace("<NULL>", "nil")
                    .replace("<TRUE>", "true")
                    .replace("<FALSE>", "false")
            )?;
        }

        write!(
            writer,
            r#"
    */"#
        )?;
    }

    if is_constructor {
        write!(
            writer,
            r#"
    public init{}("#,
            if function.output.is_nullable { "?" } else { "" }
        )?;
    } else {
        write!(
            writer,
            r#"
    public{} func {}("#,
            if is_static { " static" } else { "" },
            function.method.to_mixed_case()
        )?;
    }

    for (i, (name, typ)) in function
        .inputs
        .iter()
        .skip(if is_static { 0 } else { 1 })
        .enumerate()
    {
        if i != 0 {
            write!(writer, ", ")?;
        }
        write!(writer, "_ {}: {}", name.to_mixed_case(), get_hl_type(typ))?;
    }

    if is_constructor {
        write!(
            writer,
            r#") {{
        super.init(ptr: Optional.none)
        "#
        )?;
    } else if has_return_type {
        write!(
            writer,
            r#") -> {}{} {{
        "#,
            return_type,
            if function.output.is_nullable && function.output.is_custom {
                "?"
            } else {
                ""
            }
        )?;
    } else {
        write!(
            writer,
            r#") {{
        "#
        )?;
    }

    for (name, typ) in function.inputs.iter() {
        if typ.is_custom {
            write!(
                writer,
                r#"assert({name}.ptr != Optional.none)
        "#,
                name = if name == "this" {
                    "self".to_string()
                } else {
                    name.to_mixed_case()
                }
            )?;
        }
    }

    if has_return_type {
        if !is_constructor && function.output.is_custom {
            write!(writer, r#"let result = {}(ptr: "#, return_type)?;
        } else {
            write!(writer, "let result = ")?;
        }
    }

    write!(writer, r#"LiveSplitCoreNative.{}("#, &function.name)?;

    for (i, (name, typ)) in function.inputs.iter().enumerate() {
        if i != 0 {
            write!(writer, ", ")?;
        }
        let ty_name = get_ll_type(typ);
        write!(
            writer,
            "{}",
            if name == "this" {
                "self.ptr".to_string()
            } else if typ.is_custom {
                format!("{}.ptr", name.to_mixed_case())
            } else if ty_name == "Bool" {
                format!("{} ? 1 : 0", name.to_mixed_case())
            } else {
                name.to_mixed_case()
            }
        )?;
    }

    write!(
        writer,
        "){}",
        if return_type == "Bool" { " != 0" } else { "" }
    )?;

    if !is_constructor && has_return_type && function.output.is_custom {
        write!(writer, r#")"#)?;
    }

    for (name, typ) in function.inputs.iter() {
        if typ.is_custom && typ.kind == TypeKind::Value {
            write!(
                writer,
                r#"
        {}.ptr = Optional.none"#,
                if name == "this" {
                    "self".to_string()
                } else {
                    name.to_mixed_case()
                }
            )?;
        }
    }

    if has_return_type {
        if function.output.is_nullable && function.output.is_custom {
            if is_constructor {
                write!(
                    writer,
                    r#"
        if result == Optional.none {{
            return nil
        }}"#
                )?;
            } else {
                write!(
                    writer,
                    r#"
        if result.ptr == Optional.none {{
            return Optional.none
        }}"#
                )?;
            }
        }
        if return_type == "String" {
            write!(
                writer,
                r#"
        return String(cString: result!)"#
            )?;
        } else if is_constructor {
            write!(
                writer,
                r#"
        self.ptr = result"#
            )?;
        } else {
            write!(
                writer,
                r#"
        return result"#
            )?;
        }
    }

    write!(
        writer,
        r#"
    }}"#
    )?;

    Ok(())
}

pub fn write<W: Write>(mut writer: W, classes: &BTreeMap<String, Class>) -> Result<()> {
    write!(
        writer,
        r#"import LiveSplitCoreNative
"#
    )?;

    for (class_name, class) in classes {
        let class_name_ref = format!("{}Ref", class_name);
        let class_name_ref_mut = format!("{}RefMut", class_name);

        write_class_comments(&mut writer, &class.comments)?;

        write!(
            writer,
            r#"
public class {class} {{
    var ptr: UnsafeMutableRawPointer?"#,
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
    public func readWith(_ block: (TimerRef) -> ()) {
        let lock = self.read()!
        block(lock.timer()!)
        lock.dispose()
    }
    public func writeWith(_ block: (TimerRefMut) -> ()) {
        let lock = self.write()!
        block(lock.timer()!)
        lock.dispose()
    }"#
            )?;
        }

        write!(
            writer,
            r#"
    init(ptr: UnsafeMutableRawPointer?) {{
        self.ptr = ptr
    }}
}}
"#
        )?;

        write_class_comments(&mut writer, &class.comments)?;

        write!(
            writer,
            r#"
public class {class}: {base_class} {{"#,
            class = class_name_ref_mut,
            base_class = class_name_ref
        )?;

        for function in &class.mut_fns {
            write_fn(&mut writer, function)?;
        }

        write!(
            writer,
            r#"
    override init(ptr: UnsafeMutableRawPointer?) {{
        super.init(ptr: ptr)
    }}
}}
"#
        )?;

        write_class_comments(&mut writer, &class.comments)?;

        write!(
            writer,
            r#"
public class {class} : {base_class} {{
    private func drop() {{
        if self.ptr != Optional.none {{"#,
            class = class_name,
            base_class = class_name_ref_mut
        )?;

        if let Some(function) = class.own_fns.iter().find(|f| f.method == "drop") {
            write!(
                writer,
                r#"
            LiveSplitCoreNative.{}(self.ptr)"#,
                function.name
            )?;
        }

        write!(
            writer,
            r#"
            self.ptr = Optional.none
        }}
    }}
    deinit {{
        self.drop()
    }}
    public func dispose() {{
        self.drop()
    }}"#
        )?;

        for function in class.static_fns.iter().chain(class.own_fns.iter()) {
            if function.method != "drop" {
                write_fn(&mut writer, function)?;
            }
        }

        writeln!(
            writer,
            r#"
    override init(ptr: UnsafeMutableRawPointer?) {{
        super.init(ptr: ptr)
    }}
}}"#
        )?;
    }

    Ok(())
}
