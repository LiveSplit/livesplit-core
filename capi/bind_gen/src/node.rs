use std::io::{Write, Result};
use {Class, Function, Type, TypeKind};
use heck::MixedCase;
use std::collections::BTreeMap;

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
        (TypeKind::Ref, "c_char") => "'CString'",
        (TypeKind::Ref, _) |
        (TypeKind::RefMut, _) => "'pointer'",
        (_, t) if !ty.is_custom => {
            match t {
                "i8" => "'int8'",
                "i16" => "'int16'",
                "i32" => "'int32'",
                "i64" => "'int64'",
                "u8" => "'uint8'",
                "u16" => "'uint16'",
                "u32" => "'uint32'",
                "u64" => "'uint64'",
                "usize" => "'size_t'",
                "f32" => "'float'",
                "f64" => "'double'",
                "bool" => "'bool'",
                "()" => "'void'",
                "c_char" => "'char'",
                x => x,
            }
        }
        _ => "'pointer'",
    }
}

fn write_fn<W: Write>(mut writer: W, function: &Function) -> Result<()> {
    let is_static = function.is_static();
    let has_return_type = function.has_return_type();
    let return_type = get_hl_type(&function.output);
    let return_type_ll = get_ll_type(&function.output);
    let mut method = function.method.to_mixed_case();
    if method == "new" {
        method = "create".to_string();
    }

    write!(writer,
           r#"
    {}{}("#,
           if is_static { "static " } else { "" },
           method)?;

    for (i, &(ref name, _)) in
        function.inputs.iter().skip(if is_static { 0 } else { 1 }).enumerate() {
        if i != 0 {
            write!(writer, ", ")?;
        }
        write!(writer, "{}", name.to_mixed_case())?;
    }

    write!(writer,
           r#") {{
        "#)?;

    for &(ref name, ref typ) in function.inputs.iter() {
        if typ.is_custom {
            write!(writer,
                   r#"if ({name}.ptr.isNull()) {{
            throw "{name} is disposed";
        }}
        "#,
                   name = name.to_mixed_case())?;
        }
    }

    if has_return_type {
        if function.output.is_custom {
            write!(writer, r#"var result = new {}("#, return_type)?;
        } else {
            write!(writer, "var result = ")?;
            if return_type_ll == "UIntPtr" {
                write!(writer, "(long)")?;
            }
        }
    }

    write!(writer, r#"liveSplitCoreNative.{}("#, &function.name)?;

    for (i, &(ref name, ref typ)) in function.inputs.iter().enumerate() {
        if i != 0 {
            write!(writer, ", ")?;
        }
        write!(writer,
               "{}",
               if name == "this" {
                   "this.ptr".to_string()
               } else if typ.is_custom {
                   format!("{}.ptr", name.to_mixed_case())
               } else {
                   name.to_mixed_case()
               })?;
    }

    write!(writer, ")")?;

    if has_return_type && function.output.is_custom {
        write!(writer, r#")"#)?;
    }

    write!(writer, r#";"#)?;

    for &(ref name, ref typ) in function.inputs.iter() {
        if typ.is_custom && typ.kind == TypeKind::Value {
            write!(writer,
                   r#"
        {}.ptr = ref.NULL;"#,
                   name.to_mixed_case())?;
        }
    }

    if has_return_type {
        if function.output.is_custom {
            write!(writer,
                   r#"
        if (result.ptr.isNull()) {{
            return null;
        }}"#)?;
        }
        write!(writer,
               r#"
        return result;"#)?;
    }

    write!(writer,
           r#"
    }}"#)?;

    Ok(())
}

pub fn write<W: Write>(mut writer: W, classes: &BTreeMap<String, Class>) -> Result<()> {
    write!(writer,
           "{}",
           r#""use strict";
var ffi = require('ffi');
var fs = require('fs');
var ref = require('ref');

var liveSplitCoreNative = ffi.Library('livesplit_core', {"#)?;

    for class in classes.values() {
        for function in class.static_fns
            .iter()
            .chain(class.own_fns.iter())
            .chain(class.shared_fns.iter())
            .chain(class.mut_fns.iter()) {
            write!(writer,
                   r#"
    '{}': [{}, ["#,
                   function.name,
                   get_ll_type(&function.output))?;

            for (i, &(_, ref typ)) in function.inputs.iter().enumerate() {
                if i != 0 {
                    write!(writer, ", ")?;
                }
                write!(writer, "{}", get_ll_type(typ))?;
            }

            write!(writer, "]],")?;
        }
    }

    writeln!(writer,
             "{}",
             r#"
});"#)?;

    for (class_name, class) in classes {
        let class_name_ref = format!("{}Ref", class_name);
        let class_name_ref_mut = format!("{}RefMut", class_name);

        write!(writer,
               r#"
class {class} {{"#,
               class = class_name_ref)?;

        for function in &class.shared_fns {
            write_fn(&mut writer, function)?;
        }

        if class_name == "SharedTimer" {
            write!(writer,
                   "{}",
                   r#"
    readWith(action) {
        this.read().with(function (lock) {
            action(lock.timer());
        });
    }
    writeWith(action) {
        this.write().with(function (lock) {
            action(lock.timer());
        });
    }"#)?;
        }

        write!(writer,
               r#"
    constructor(ptr) {{
        this.ptr = ptr;
    }}
}}
exports.{base_class} = {base_class};

class {class} extends {base_class} {{"#,
               class = class_name_ref_mut,
               base_class = class_name_ref)?;

        for function in &class.mut_fns {
            write_fn(&mut writer, function)?;
        }

        write!(writer,
               r#"
    constructor(ptr) {{
        super(ptr);
    }}
}}
exports.{base_class} = {base_class};

class {class} extends {base_class} {{
    with(closure) {{
        try {{
            closure(this);
        }} finally {{
            this.dispose();
        }}
    }}
    dispose() {{
        if (!this.ptr.isNull()) {{"#,
               class = class_name,
               base_class = class_name_ref_mut)?;

        if let Some(function) = class.own_fns.iter().find(|f| f.method == "drop") {
            write!(writer,
                   r#"
            liveSplitCoreNative.{}(this.ptr);"#,
                   function.name)?;
        }

        write!(writer,
               r#"
            this.ptr = ref.NULL;
        }}
    }}"#)?;

        for function in class.static_fns
            .iter()
            .chain(class.own_fns.iter()) {
            if function.method != "drop" {
                write_fn(&mut writer, function)?;
            }
        }

        if class_name == "Run" {
            write!(writer,
                   "{}",
                   r#"
    static parseFile(file) {
        var data = fs.readFileSync(file);
        return Run.parse(data, data.byteLength);
    }"#)?;
        }

        writeln!(writer,
                 r#"
    constructor(ptr) {{
        super(ptr);
    }}
}}
exports.{class} = {class};"#,
                 class = class_name)?;
    }

    Ok(())
}
