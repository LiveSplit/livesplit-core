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
        "Boolean".to_string()
    } else if ty.name == "usize" || ty.name == "isize" {
        "Long".to_string()
    } else {
        get_ll_type(ty).to_string()
    }
}

fn get_ll_type(ty: &Type) -> &str {
    match (ty.kind, ty.name.as_str()) {
        (TypeKind::Ref, "c_char") => "String",
        (TypeKind::Ref, _) | (TypeKind::RefMut, _) => "Long",
        (_, t) if !ty.is_custom => match t {
            "i8" => "Byte",
            "i16" => "Short",
            "i32" => "Int",
            "i64" => "Long",
            "u8" => "Byte",
            "u16" => "Short",
            "u32" => "Int",
            "u64" => "Long",
            "usize" => "Long",
            "isize" => "Long",
            "f32" => "Float",
            "f64" => "Double",
            "bool" => "Boolean",
            "()" => "Unit",
            "c_char" => "Byte",
            "Json" => "String",
            x => x,
        },
        _ => "Long",
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
    )
}

fn write_fn<W: Write>(mut writer: W, function: &Function) -> Result<()> {
    let is_static = function.is_static();
    let has_return_type = function.has_return_type();
    let return_type = get_hl_type(&function.output);
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
    constructor("#
        )?;
    } else {
        write!(
            writer,
            r#"
    fun {}("#,
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
            "{}: {}",
            name.to_lower_camel_case(),
            get_hl_type(typ)
        )?;
    }

    if is_constructor {
        write!(
            writer,
            r#"): super(0L) {{
        "#
        )?;
    } else if has_return_type {
        write!(writer, r#"): {}"#, return_type)?;
        if function.output.is_nullable && function.output.is_custom {
            write!(writer, "?")?;
        }
        write!(
            writer,
            r#" {{
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
                r#"if ({name}.ptr == 0L) {{
            throw RuntimeException()
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
                r#"val result = {ret_type}("#,
                ret_type = return_type
            )?;
        } else {
            write!(writer, "val result = ")?;
        }
    }

    write!(
        writer,
        r#"LiveSplitCoreNative.{}_{}("#,
        function.class,
        function.method.to_lower_camel_case()
    )?;

    for (i, (name, typ)) in function.inputs.iter().enumerate() {
        if i != 0 {
            write!(writer, ", ")?;
        }
        write!(
            writer,
            "{}",
            if name == "this" {
                "this.ptr".to_string()
            } else if typ.is_custom {
                format!("{}.ptr", name.to_lower_camel_case())
            } else {
                name.to_lower_camel_case()
            }
        )?;
    }

    write!(writer, ")")?;

    if !is_constructor && has_return_type && function.output.is_custom {
        write!(writer, r#")"#)?;
    }

    for (name, typ) in function.inputs.iter() {
        if typ.is_custom && typ.kind == TypeKind::Value {
            write!(
                writer,
                r#"
        {}.ptr = 0L"#,
                name.to_lower_camel_case()
            )?;
        }
    }

    if has_return_type && !is_constructor {
        if function.output.is_nullable && function.output.is_custom {
            write!(
                writer,
                r#"
        if (result.ptr == 0L) {{
            return null
        }}"#
            )?;
        }
        write!(
            writer,
            r#"
        return result"#
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

    writeln!(writer, r#"package livesplitcore"#)?;

    write_class_comments(&mut writer, &class.comments)?;

    write!(
        writer,
        r#"
open class {class} internal constructor(var ptr: Long) {{"#,
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
    fun readWith(action: (TimerRef) -> Unit) {
        val lock = read()!!
        lock.use { lock ->
            action(lock.timer()!!)
        }
    }
    fun writeWith(action: (TimerRefMut) -> Unit) {
        val lock = write()!!
        lock.use { lock ->
            action(lock.timer()!!)
        }
    }"#
        )?;
    }

    write!(
        writer,
        r#"
}}"#
    )
}

fn write_class_ref_mut<P: AsRef<Path>>(path: P, class_name: &str, class: &Class) -> Result<()> {
    let mut writer = BufWriter::new(File::create(path)?);
    let class_name_ref = format!("{}Ref", class_name);
    let class_name_ref_mut = format!("{}RefMut", class_name);

    writeln!(writer, r#"package livesplitcore"#)?;

    write_class_comments(&mut writer, &class.comments)?;

    write!(
        writer,
        r#"
open class {class} internal constructor(ptr: Long) : {base_class}(ptr) {{"#,
        class = class_name_ref_mut,
        base_class = class_name_ref
    )?;

    for function in &class.mut_fns {
        write_fn(&mut writer, function)?;
    }

    write!(
        writer,
        r#"
}}"#
    )
}

fn write_class<P: AsRef<Path>>(path: P, class_name: &str, class: &Class) -> Result<()> {
    let mut writer = BufWriter::new(File::create(path)?);
    let class_name_ref_mut = format!("{}RefMut", class_name);

    writeln!(writer, r#"package livesplitcore"#)?;

    write_class_comments(&mut writer, &class.comments)?;

    write!(
        writer,
        r#"
open class {class} : {base_class}, AutoCloseable {{
    private fun drop() {{
        if (ptr != 0L) {{"#,
        class = class_name,
        base_class = class_name_ref_mut
    )?;

    if let Some(function) = class.own_fns.iter().find(|f| f.method == "drop") {
        write!(
            writer,
            r#"
            LiveSplitCoreNative.{}(this.ptr)"#,
            function.name
        )?;
    }

    write!(
        writer,
        r#"
            ptr = 0
        }}
    }}
    protected fun finalize() {{
        drop()
    }}
    override fun close() {{
        drop()
    }}"#
    )?;

    for function in &class.own_fns {
        if function.method != "drop" {
            write_fn(&mut writer, function)?;
        }
    }

    let mut constructor = None;

    if !class.static_fns.is_empty() {
        write!(
            writer,
            r#"
    companion object {{"#
        )?;

        for function in &class.static_fns {
            if function.method != "new" || function.output.is_nullable {
                write_fn(&mut writer, function)?;
            } else {
                constructor = Some(function);
            }
        }

        if class_name == "Run" {
            write!(
                writer,
                "{}",
                r#"
    fun parse(data: String, loadFilesPath: String): ParseRunResult {
        val result = ParseRunResult(LiveSplitCoreNative.Run_parseString(data, loadFilesPath))
        return result
    }"#
            )?;
        }

        write!(
            writer,
            r#"
    }}"#
        )?;
    }

    if let Some(constructor) = constructor {
        write_fn(&mut writer, constructor)?;
    }

    write!(
        writer,
        r#"
    internal constructor(ptr: Long) : super(ptr) {{}}
}}"#
    )
}

fn write_native_class<P: AsRef<Path>>(path: P, classes: &BTreeMap<String, Class>) -> Result<()> {
    let mut writer = BufWriter::new(File::create(path)?);

    write!(
        writer,
        "{}",
        r#"package livesplitcore

object LiveSplitCoreNative {
    init {
        System.loadLibrary("native-lib")
    }
    external fun Run_parseString(data: String): Long"#
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
    external fun {}_{}("#,
                function.class,
                function.method.to_lower_camel_case()
            )?;

            for (i, (name, typ)) in function.inputs.iter().enumerate() {
                if i != 0 {
                    write!(writer, ", ")?;
                }
                write!(
                    writer,
                    "{}: {}",
                    if name == "this" {
                        String::from("self")
                    } else {
                        name.to_lower_camel_case()
                    },
                    get_ll_type(typ)
                )?;
            }

            write!(writer, ")")?;

            if function.has_return_type() {
                write!(writer, ": {}", get_ll_type(&function.output))?;
            }
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

    path.push("LiveSplitCoreNative.kt");
    write_native_class(&path, classes)?;
    path.pop();

    for (class_name, class) in classes {
        path.push(format!("{}Ref", class_name));
        path.set_extension("kt");
        write_class_ref(&path, class_name, class)?;
        path.pop();

        path.push(format!("{}RefMut", class_name));
        path.set_extension("kt");
        write_class_ref_mut(&path, class_name, class)?;
        path.pop();

        path.push(class_name);
        path.set_extension("kt");
        write_class(&path, class_name, class)?;
        path.pop();
    }

    Ok(())
}
