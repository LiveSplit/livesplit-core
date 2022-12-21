use crate::{Class, Function, Type, TypeKind};
use heck::ToLowerCamelCase;
use std::{
    borrow::Cow,
    collections::BTreeMap,
    io::{Result, Write},
};

fn get_c_type(ty: &Type) -> Cow<'_, str> {
    let mut name = Cow::Borrowed(match ty.name.as_str() {
        "i8" => "int8_t",
        "i16" => "int16_t",
        "i32" => "int32_t",
        "i64" => "int64_t",
        "u8" => "uint8_t",
        "u16" => "uint16_t",
        "u32" => "uint32_t",
        "u64" => "uint64_t",
        "usize" => "size_t",
        "isize" => "ssize_t",
        "f32" => "float",
        "f64" => "double",
        "bool" => "bool",
        "()" => "void",
        "c_char" => "char",
        "Json" => "char const*",
        x => x,
    });
    match (ty.is_custom, ty.kind) {
        (false, TypeKind::RefMut) => name.to_mut().push_str("*restrict"),
        (false, TypeKind::Ref) => name.to_mut().push_str(" const*"),
        (true, TypeKind::RefMut) => name.to_mut().push_str("RefMut"),
        (true, TypeKind::Ref) => name.to_mut().push_str("Ref"),
        _ => (),
    }
    if name == "uint8_t const*" {
        name = Cow::Borrowed("void const*");
    }
    name
}

fn get_jni_type(ty: &Type) -> &str {
    match (ty.kind, ty.name.as_str()) {
        (TypeKind::Ref, "c_char") => "jstring",
        (TypeKind::Ref, _) | (TypeKind::RefMut, _) => "jlong",
        (_, t) if !ty.is_custom => match t {
            "i8" => "jbyte",
            "i16" => "jshort",
            "i32" => "jint",
            "i64" => "jlong",
            "u8" => "jbyte",
            "u16" => "jshort",
            "u32" => "jint",
            "u64" => "jlong",
            "usize" => "jlong",
            "isize" => "jlong",
            "f32" => "jfloat",
            "f64" => "jdouble",
            "bool" => "jboolean",
            "()" => "void",
            "c_char" => "jbyte",
            "Json" => "jstring",
            x => x,
        },
        _ => "jlong",
    }
}

fn write_fn<W: Write>(mut writer: W, function: &Function, class_name: &str) -> Result<()> {
    let has_return_type = function.has_return_type();
    let return_type = get_jni_type(&function.output);

    write!(
        writer,
        r#"
extern "C" JNIEXPORT {} Java_livesplitcore_LiveSplitCoreNative_{}_1{}(JNIEnv* jni_env, jobject"#,
        return_type,
        class_name,
        function.method.to_lower_camel_case()
    )?;

    for (name, typ) in &function.inputs {
        write!(
            writer,
            ", {} {}",
            get_jni_type(typ),
            if name == "this" { "self" } else { name }
        )?;
    }

    write!(
        writer,
        r#") {{
    "#
    )?;

    for (name, typ) in &function.inputs {
        let jni_type = get_jni_type(typ);
        if jni_type == "jstring" {
            write!(
                writer,
                r#"auto cstr_{name} = jni_env->GetStringUTFChars({name}, nullptr);
    "#,
                name = name
            )?;
        }
    }

    if has_return_type {
        if return_type == "jstring" {
            write!(writer, r#"auto result = jni_env->NewStringUTF("#)?;
        } else {
            write!(writer, r#"auto result = ({})("#, return_type)?;
        }
    }

    write!(writer, r#"{}("#, &function.name)?;

    for (i, (name, typ)) in function.inputs.iter().enumerate() {
        if i != 0 {
            write!(writer, ", ")?;
        }
        let ty_name = get_c_type(typ);
        let jni_type = get_jni_type(typ);
        write!(
            writer,
            "{}",
            if name == "this" {
                format!("({})self", ty_name)
            } else if jni_type == "jstring" {
                format!("cstr_{}", name)
            } else {
                format!("({}){}", ty_name, name)
            }
        )?;
    }

    write!(writer, ")")?;

    if has_return_type {
        write!(writer, r#")"#)?;
    }

    write!(writer, r#";"#)?;

    for (name, typ) in &function.inputs {
        let jni_type = get_jni_type(typ);
        if jni_type == "jstring" {
            write!(
                writer,
                r#"
    jni_env->ReleaseStringUTFChars({name}, cstr_{name});"#,
                name = name
            )?;
        }
    }

    if has_return_type {
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

pub fn write<W: Write>(mut writer: W, classes: &BTreeMap<String, Class>) -> Result<()> {
    write!(
        writer,
        "{}",
        r#"#include <jni.h>
#include <string>
#include "livesplit_core.h"

using namespace LiveSplit;

extern "C" JNIEXPORT jlong Java_livesplitcore_LiveSplitCoreNative_Run_1parseString(JNIEnv* jni_env, jobject, jstring data, jstring load_files_path) {
    auto cstr_data = jni_env->GetStringUTFChars(data, nullptr);
    auto cstr_load_files_path = jni_env->GetStringUTFChars(load_files_path, nullptr);
    auto result = (jlong)Run_parse(cstr_data, strlen(cstr_data), cstr_load_files_path);
    jni_env->ReleaseStringUTFChars(load_files_path, cstr_load_files_path);
    jni_env->ReleaseStringUTFChars(data, cstr_data);
    return result;
}
"#
    )?;

    for (class_name, class) in classes {
        for function in class
            .static_fns
            .iter()
            .chain(class.own_fns.iter())
            .chain(class.shared_fns.iter())
            .chain(class.mut_fns.iter())
        {
            write_fn(&mut writer, function, class_name)?;
            writeln!(writer)?;
        }
    }

    Ok(())
}
