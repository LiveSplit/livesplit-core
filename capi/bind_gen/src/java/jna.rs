use super::write_class_comments;
use crate::{Class, Function, Type, TypeKind};
use heck::ToLowerCamelCase;
use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufWriter, Result, Write},
    path::Path,
};

fn get_hl_type(ty: &Type) -> String {
    if ty.is_custom {
        match ty.kind {
            TypeKind::Ref => format!("{}Ref", ty.name),
            TypeKind::RefMut => format!("{}RefMut", ty.name),
            TypeKind::Value => ty.name.clone(),
        }
    } else if ty.name == "bool" {
        "boolean".to_string()
    } else if ty.name == "usize" || ty.name == "isize" {
        "long".to_string()
    } else {
        get_ll_type(ty).to_string()
    }
}

fn get_ll_type(ty: &Type) -> &str {
    match (ty.kind, ty.name.as_str()) {
        (TypeKind::Ref, "c_char") => "String",
        (TypeKind::Ref, _) | (TypeKind::RefMut, _) => "Pointer",
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
                "isize" => "NativeLong", // Not really correct
                "f32" => "float",
                "f64" => "double",
                "bool" => "byte",
                "()" => "void",
                "c_char" => "byte",
                "Json" => "String",
                x => x,
            }
        }
        _ => "Pointer",
    }
}

fn write_fn<W: Write>(mut writer: W, function: &Function, class_name: &str) -> Result<()> {
    let is_static = function.is_static();
    let has_return_type = function.has_return_type();
    let return_type = get_hl_type(&function.output);
    let return_type_ll = get_ll_type(&function.output);
    let is_constructor = function.method == "new" && !function.output.is_nullable;
    let mut method = function.method.to_lower_camel_case();
    if method == "clone" {
        method = "copy".into();
    } else if method == "close" {
        method = "finish".into();
    } else if method == "new" {
        method = "create".into();
    } else if method == "default" {
        method = "createDefault".into();
    }

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
     * {}"#,
                comment
                    .replace("<NULL>", "null")
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
    public {}("#,
            class_name
        )?;
    } else {
        write!(
            writer,
            r#"
    public{} {} {}("#,
            if is_static { " static" } else { "" },
            return_type,
            method
        )?;
    }

    for (i, (name, typ)) in function
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
            "{} {}",
            get_hl_type(typ),
            name.to_lower_camel_case()
        )?;
    }

    if is_constructor {
        write!(
            writer,
            r#") {{
        super(Pointer.NULL);
        "#
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
                r#"if ({name}.ptr == Pointer.NULL) {{
            throw new RuntimeException();
        }}
        "#,
                name = name.to_lower_camel_case()
            )?;
        }
    }

    if has_return_type {
        if is_constructor {
            write!(writer, "this.ptr = ")?;
        } else if function.output.is_custom {
            write!(
                writer,
                r#"{ret_type} result = new {ret_type}("#,
                ret_type = return_type
            )?;
        } else {
            write!(writer, "{} result = ", return_type)?;
        }
    }

    write!(
        writer,
        r#"LiveSplitCoreNative.INSTANCE.{}("#,
        &function.name
    )?;

    for (i, (name, typ)) in function.inputs.iter().enumerate() {
        if i != 0 {
            write!(writer, ", ")?;
        }
        let hl_ty_name = get_hl_type(typ);
        let ty_name = get_ll_type(typ);
        write!(
            writer,
            "{}",
            if name == "this" {
                "this.ptr".to_string()
            } else if hl_ty_name == "boolean" {
                format!("(byte)({} ? 1 : 0)", name.to_lower_camel_case())
            } else if ty_name == "NativeLong" {
                format!("new NativeLong({})", name.to_lower_camel_case())
            } else if typ.is_custom {
                format!("{}.ptr", name.to_lower_camel_case())
            } else {
                name.to_lower_camel_case()
            }
        )?;
    }

    write!(
        writer,
        "){}",
        if return_type == "boolean" {
            " != 0"
        } else if return_type_ll == "NativeLong" {
            ".longValue()"
        } else {
            ""
        }
    )?;

    if !is_constructor && has_return_type && function.output.is_custom {
        write!(writer, r#")"#)?;
    }

    write!(writer, r#";"#)?;

    for (name, typ) in function.inputs.iter() {
        if typ.is_custom && typ.kind == TypeKind::Value {
            write!(
                writer,
                r#"
        {}.ptr = Pointer.NULL;"#,
                name.to_lower_camel_case()
            )?;
        }
    }

    if has_return_type && !is_constructor {
        if function.output.is_nullable && function.output.is_custom {
            write!(
                writer,
                r#"
        if (result.ptr == Pointer.NULL) {{
            return null;
        }}"#
            )?;
        }
        write!(
            writer,
            r#"
        return result;"#
        )?;
    }

    write!(
        writer,
        r#"
    }}"#
    )?;

    Ok(())
}

fn write_class_ref<P: AsRef<Path>>(path: P, class_name: &str, class: &Class) -> Result<()> {
    let mut writer = BufWriter::new(File::create(path)?);
    let class_name_ref = format!("{}Ref", class_name);

    write!(
        writer,
        r#"package livesplitcore;

import com.sun.jna.*;
"#
    )?;

    write_class_comments(&mut writer, &class.comments)?;

    write!(
        writer,
        r#"
public class {class} {{
    Pointer ptr;"#,
        class = class_name_ref
    )?;

    for function in &class.shared_fns {
        write_fn(&mut writer, function, &class_name_ref)?;
    }

    if class_name == "SharedTimer" {
        write!(
            writer,
            "{}",
            r#"
    public void readWith(java.util.function.Consumer<TimerRef> action) {
        try (TimerReadLock timerLock = read()) {
            action.accept(timerLock.timer());
        }
    }
    public void writeWith(java.util.function.Consumer<TimerRefMut> action) {
        try (TimerWriteLock timerLock = write()) {
            action.accept(timerLock.timer());
        }
    }"#
        )?;
    }

    write!(
        writer,
        r#"
    {class}(Pointer ptr) {{
        this.ptr = ptr;
    }}
}}"#,
        class = class_name_ref
    )
}

fn write_class_ref_mut<P: AsRef<Path>>(path: P, class_name: &str, class: &Class) -> Result<()> {
    let mut writer = BufWriter::new(File::create(path)?);
    let class_name_ref = format!("{}Ref", class_name);
    let class_name_ref_mut = format!("{}RefMut", class_name);

    write!(
        writer,
        r#"package livesplitcore;

import com.sun.jna.*;
"#
    )?;

    write_class_comments(&mut writer, &class.comments)?;

    write!(
        writer,
        r#"
public class {class} extends {base_class} {{"#,
        class = class_name_ref_mut,
        base_class = class_name_ref
    )?;

    for function in &class.mut_fns {
        write_fn(&mut writer, function, class_name)?;
    }

    write!(
        writer,
        r#"
    {class}(Pointer ptr) {{
        super(ptr);
    }}
}}"#,
        class = class_name_ref_mut
    )
}

fn write_class<P: AsRef<Path>>(path: P, class_name: &str, class: &Class) -> Result<()> {
    let mut writer = BufWriter::new(File::create(path)?);
    let class_name_ref_mut = format!("{}RefMut", class_name);

    write!(
        writer,
        r#"package livesplitcore;

import com.sun.jna.*;
"#
    )?;

    write_class_comments(&mut writer, &class.comments)?;

    write!(
        writer,
        r#"
public class {class} extends {base_class} implements AutoCloseable {{
    private void drop() {{
        if (ptr != Pointer.NULL) {{"#,
        class = class_name,
        base_class = class_name_ref_mut
    )?;

    if let Some(function) = class.own_fns.iter().find(|f| f.method == "drop") {
        write!(
            writer,
            r#"
            LiveSplitCoreNative.INSTANCE.{}(this.ptr);"#,
            function.name
        )?;
    }

    write!(
        writer,
        r#"
            ptr = Pointer.NULL;
        }}
    }}
    protected void finalize() throws Throwable {{
        drop();
        super.finalize();
    }}
    public void close() {{
        drop();
    }}"#
    )?;

    for function in class.static_fns.iter().chain(class.own_fns.iter()) {
        if function.method != "drop" {
            write_fn(&mut writer, function, class_name)?;
        }
    }

    if class_name == "Run" {
        write!(
            writer,
            "{}",
            r#"
    public static ParseRunResult parse(java.io.InputStream stream, String loadFilesPath) throws java.io.IOException {
        java.io.ByteArrayOutputStream out = new java.io.ByteArrayOutputStream();
        byte[] buffer = new byte[1024];
        while (true) {
            int r = stream.read(buffer);
            if (r == -1) break;
            out.write(buffer, 0, r);
        }
        byte[] arr = out.toByteArray();
        java.nio.ByteBuffer nativeBuf = java.nio.ByteBuffer.allocateDirect(arr.length);
        nativeBuf.put(arr);
        return Run.parse(Native.getDirectBufferPointer(nativeBuf), arr.length, loadFilesPath);
    }"#
        )?;
    }

    write!(
        writer,
        r#"
    {class}(Pointer ptr) {{
        super(ptr);
    }}
}}"#,
        class = class_name
    )
}

fn write_native_class<P: AsRef<Path>>(path: P, classes: &BTreeMap<String, Class>) -> Result<()> {
    let mut writer = BufWriter::new(File::create(path)?);

    writeln!(
        writer,
        "{}",
        r#"package livesplitcore;

import com.sun.jna.*;

public interface LiveSplitCoreNative extends Library {
    LiveSplitCoreNative INSTANCE = (LiveSplitCoreNative) Native.loadLibrary("livesplit_core", LiveSplitCoreNative.class);"#
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
    {} {}("#,
                get_ll_type(&function.output),
                function.name
            )?;

            for (i, (name, typ)) in function.inputs.iter().enumerate() {
                if i != 0 {
                    write!(writer, ", ")?;
                }
                write!(
                    writer,
                    "{} {}",
                    get_ll_type(typ),
                    if name == "this" {
                        String::from("self")
                    } else {
                        name.to_lower_camel_case()
                    }
                )?;
            }

            write!(writer, ");")?;
        }
    }

    writeln!(
        writer,
        "{}",
        r#"
}"#
    )
}

pub fn write<P: AsRef<Path>>(path: P, classes: &BTreeMap<String, Class>) -> Result<()> {
    let mut path = path.as_ref().to_owned();

    path.push("LiveSplitCoreNative.java");
    write_native_class(&path, classes)?;
    path.pop();

    for (class_name, class) in classes {
        path.push(format!("{}Ref", class_name));
        path.set_extension("java");
        write_class_ref(&path, class_name, class)?;
        path.pop();

        path.push(format!("{}RefMut", class_name));
        path.set_extension("java");
        write_class_ref_mut(&path, class_name, class)?;
        path.pop();

        path.push(class_name);
        path.set_extension("java");
        write_class(&path, class_name, class)?;
        path.pop();
    }

    Ok(())
}
