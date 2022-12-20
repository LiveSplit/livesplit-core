use crate::{Class, Function, Type, TypeKind};
use heck::ToLowerCamelCase;
use std::{
    borrow::Cow,
    collections::BTreeMap,
    fmt::Write as _,
    io::{Result, Write},
};

fn get_hl_type(ty: &Type) -> Cow<str> {
    if ty.is_custom {
        match ty.kind {
            TypeKind::Ref => Cow::Owned(format!("{}Ref", ty.name)),
            TypeKind::RefMut => Cow::Owned(format!("{}RefMut", ty.name)),
            TypeKind::Value => Cow::Borrowed(&ty.name),
        }
    } else {
        Cow::Borrowed(get_ll_type(ty))
    }
}

fn get_ll_type(ty: &Type) -> &str {
    match (ty.kind, ty.name.as_str()) {
        (TypeKind::Ref, "c_char") => "String",
        (TypeKind::Ref | TypeKind::RefMut, _) => "UnsafeMutableRawPointer?",
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
            "isize" => "ptrdiff_t",
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

fn get_input_name(name: &str) -> Cow<str> {
    if name == "this" {
        Cow::Borrowed("self")
    } else {
        Cow::Owned(name.to_lower_camel_case())
    }
}

fn write_class_comments<W: Write>(mut writer: W, comment: &[String]) -> Result<String> {
    let mut result = String::with_capacity(comment.iter().map(String::len).sum());
    for line in comment {
        let start_index = result.len();
        write!(
            result,
            "
/// {}",
            line.replace("<NULL>", "nil")
                .replace("<TRUE>", "true")
                .replace("<FALSE>", "false")
        )
        .unwrap();
        write!(writer, "{}", &result[start_index..])?;
    }

    Ok(result)
}

fn write_fn<W: Write>(mut writer: W, function: &Function) -> Result<()> {
    let is_static = function.is_static();
    let has_return_type = function.has_return_type();
    let return_type = get_hl_type(&function.output);
    let is_constructor = function.method == "new";

    if !function.comments.is_empty() {
        for comment in &function.comments {
            write!(
                writer,
                "
    /// {}",
                comment
                    .replace("<NULL>", "nil")
                    .replace("<TRUE>", "true")
                    .replace("<FALSE>", "false")
            )?;
        }
    }

    let mut name = function.method.to_lower_camel_case();
    if name == "default" {
        name.clear();
        name.push_str("`default`");
    }

    let return_suffix = if function.output.is_nullable { "?" } else { "" };
    if is_constructor {
        write!(
            writer,
            "
    public init{}(",
            return_suffix
        )?;
    } else {
        write!(
            writer,
            "
    public{} func {}(",
            if is_static { " static" } else { "" },
            name
        )?;
    }

    for (i, (name, ty)) in function
        .inputs
        .iter()
        .skip(usize::from(!is_static))
        .enumerate()
    {
        if i != 0 {
            write!(writer, ", ")?;
        }
        write!(
            writer,
            "_ {}: {}",
            name.to_lower_camel_case(),
            get_hl_type(ty)
        )?;
    }

    if is_constructor {
        write!(
            writer,
            ") {{
        "
        )?;
    } else if has_return_type {
        write!(
            writer,
            ") -> {}{} {{
        ",
            return_type, return_suffix
        )?;
    } else {
        write!(
            writer,
            ") {{
        "
        )?;
    }

    for (name, ty) in &function.inputs {
        if ty.is_custom {
            write!(
                writer,
                "assert({}.ptr != nil)
        ",
                get_input_name(name)
            )?;
        }
    }

    if has_return_type {
        write!(writer, "let result = ")?;
    }

    write!(writer, "CLiveSplitCore.{}(", &function.name)?;

    for (i, (name, ty)) in function.inputs.iter().enumerate() {
        if i != 0 {
            write!(writer, ", ")?;
        }

        if name == "this" {
            write!(writer, "self.ptr")?;
        } else {
            write!(writer, "{}", name.to_lower_camel_case())?;
            if ty.is_custom {
                write!(writer, ".ptr")?;
            }
        }
    }

    write!(writer, ")")?;

    for (name, ty) in &function.inputs {
        if ty.is_custom && ty.kind == TypeKind::Value {
            write!(
                writer,
                "
        {}.ptr = nil",
                get_input_name(name)
            )?;
        }
    }

    if has_return_type {
        if function.output.is_nullable {
            if return_type == "String" {
                return write!(
                    writer,
                    "
        if let result = result {{
            return String(cString: result)
        }}
        return nil
    }}"
                );
            }

            write!(
                writer,
                "
        if result == nil {{
            return nil
        }}"
            )?;
        }

        if return_type == "String" {
            write!(
                writer,
                "
        return String(cString: result!)"
            )?;
        } else if is_constructor {
            write!(
                writer,
                "
        super.init(ptr: result)
    }}
    override init(ptr: UnsafeMutableRawPointer?) {{
        super.init(ptr: ptr)"
            )?;
        } else if function.output.is_custom {
            write!(
                writer,
                "
        return {}(ptr: result)",
                return_type,
            )?;
        } else {
            write!(
                writer,
                "
        return result"
            )?;
        }
    }

    write!(
        writer,
        "
    }}"
    )
}

pub fn write<W: Write>(mut writer: W, classes: &BTreeMap<String, Class>) -> Result<()> {
    writeln!(writer, "import CLiveSplitCore")?;

    for (class_name, class) in classes {
        let class_name_ref = format!("{}Ref", class_name);
        let class_name_ref_mut = format!("{}RefMut", class_name);

        let comment = write_class_comments(&mut writer, &class.comments)?;

        write!(
            writer,
            "
public class {class} {{
    var ptr: UnsafeMutableRawPointer?
    init(ptr: UnsafeMutableRawPointer?) {{
        self.ptr = ptr
    }}",
            class = class_name_ref
        )?;

        for function in &class.shared_fns {
            write_fn(&mut writer, function)?;
        }

        if class_name == "SharedTimer" {
            write!(
                writer,
                "
    public func readWith(_ block: (TimerRef) -> ()) {{
        let lock = self.read()
        block(lock.timer())
        lock.dispose()
    }}
    public func writeWith(_ block: (TimerRefMut) -> ()) {{
        let lock = self.write()
        block(lock.timer())
        lock.dispose()
    }}"
            )?;
        }

        write!(
            writer,
            "
}}
{comment}
public class {class}: {base_class} {{",
            comment = comment,
            class = class_name_ref_mut,
            base_class = class_name_ref
        )?;

        for function in &class.mut_fns {
            write_fn(&mut writer, function)?;
        }

        write!(
            writer,
            "
}}
{comment}
public class {class} : {base_class} {{
    private func drop() {{
        if self.ptr != nil {{",
            comment = comment,
            class = class_name,
            base_class = class_name_ref_mut
        )?;

        if let Some(function) = class.own_fns.iter().find(|f| f.method == "drop") {
            write!(
                writer,
                "
            CLiveSplitCore.{}(self.ptr)",
                function.name
            )?;
        }

        write!(
            writer,
            "
            self.ptr = nil
        }}
    }}
    deinit {{
        self.drop()
    }}
    public func dispose() {{
        self.drop()
    }}"
        )?;

        for function in class.static_fns.iter().chain(class.own_fns.iter()) {
            if function.method != "drop" {
                write_fn(&mut writer, function)?;
            }
        }

        writeln!(
            writer,
            "
}}"
        )?;
    }

    Ok(())
}
