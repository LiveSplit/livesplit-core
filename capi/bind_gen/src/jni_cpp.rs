use heck::MixedCase;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::io::{Result, Write};
use {Class, Function, Type, TypeKind};

fn get_c_type(ty: &Type) -> Cow<str> {
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

fn get_jni_type<'a>(ty: &'a Type) -> &'a str {
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
        function.method.to_mixed_case()
    )?;

    for &(ref name, ref typ) in &function.inputs {
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

    for &(ref name, ref typ) in &function.inputs {
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

    for (i, &(ref name, ref typ)) in function.inputs.iter().enumerate() {
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

    for &(ref name, ref typ) in &function.inputs {
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
    write!(writer,
           "{}",
           r#"#include <jni.h>
#include <string>
#include "livesplit_core.h"

using namespace LiveSplit;

extern "C" JNIEXPORT jlong Java_livesplitcore_LiveSplitCoreNative_Run_1parseString(JNIEnv* jni_env, jobject, jstring data, jstring path, jboolean load_files) {
    auto cstr_data = jni_env->GetStringUTFChars(data, nullptr);
    auto cstr_path = jni_env->GetStringUTFChars(path, nullptr);
    auto result = (jlong)Run_parse(cstr_data, strlen(cstr_data), cstr_path, (uint8_t)load_files);
    jni_env->ReleaseStringUTFChars(path, cstr_path);
    jni_env->ReleaseStringUTFChars(data, cstr_data);
    return result;
}
"#)?;

    for (class_name, class) in classes {
        for function in class
            .static_fns
            .iter()
            .chain(class.own_fns.iter())
            .chain(class.shared_fns.iter())
            .chain(class.mut_fns.iter())
        {
            write_fn(&mut writer, function, class_name)?;
            writeln!(writer, "")?;
        }
    }

    Ok(())
}
