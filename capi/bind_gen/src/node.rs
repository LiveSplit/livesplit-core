use crate::{typescript, Class, Function, Type, TypeKind};
use heck::ToLowerCamelCase;
use std::{
    collections::BTreeMap,
    io::{Result, Write},
};

fn get_hl_type_with_null(ty: &Type) -> String {
    let mut formatted = get_hl_type_without_null(ty);
    if ty.is_nullable {
        formatted.push_str(" | null");
    }
    formatted
}

fn get_hl_type_without_null(ty: &Type) -> String {
    if ty.is_custom {
        match ty.kind {
            TypeKind::Ref => format!("{}Ref", ty.name),
            TypeKind::RefMut => format!("{}RefMut", ty.name),
            TypeKind::Value => ty.name.clone(),
        }
    } else {
        match (ty.kind, ty.name.as_str()) {
            (TypeKind::Ref, "c_char") => "string",
            (TypeKind::Ref, _) | (TypeKind::RefMut, _) => "Buffer",
            (_, t) if !ty.is_custom => match t {
                "i8" => "number",
                "i16" => "number",
                "i32" => "number",
                "i64" => "number",
                "u8" => "number",
                "u16" => "number",
                "u32" => "number",
                "u64" => "number",
                "usize" => "number",
                "isize" => "number",
                "f32" => "number",
                "f64" => "number",
                "bool" => "boolean",
                "()" => "void",
                "c_char" => "string",
                "Json" => "any",
                x => x,
            },
            _ => unreachable!(),
        }
        .to_string()
    }
}

fn get_ll_type(ty: &Type) -> &str {
    match (ty.kind, ty.name.as_str()) {
        (TypeKind::Ref, "c_char") | (_, "Json") => "'CString'",
        (TypeKind::Ref, _) | (TypeKind::RefMut, _) => "'pointer'",
        (_, t) if !ty.is_custom => match t {
            "i8" => "'int8'",
            "i16" => "'int16'",
            "i32" => "'int32'",
            "i64" => "'int64'",
            "u8" => "'uint8'",
            "u16" => "'uint16'",
            "u32" => "'uint32'",
            "u64" => "'uint64'",
            "usize" => "'size_t'",
            "isize" => "'ssize_t'",
            "f32" => "'float'",
            "f64" => "'double'",
            "bool" => "'bool'",
            "()" => "'void'",
            "c_char" => "'char'",
            x => x,
        },
        _ => "'pointer'",
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

fn write_fn<W: Write>(mut writer: W, function: &Function, type_script: bool) -> Result<()> {
    let is_static = function.is_static();
    let has_return_type = function.has_return_type();
    let return_type_with_null = get_hl_type_with_null(&function.output);
    let return_type_without_null = get_hl_type_without_null(&function.output);
    let method = function.method.to_lower_camel_case();
    let is_json = has_return_type && function.output.name == "Json";

    if !function.comments.is_empty() || !type_script {
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

        if type_script {
            write!(
                writer,
                r#"
     */"#
            )?;
        }
    }

    if !type_script {
        for (name, ty) in function.inputs.iter().skip(usize::from(!is_static)) {
            write!(
                writer,
                r#"
     * @param {{{}}} {}"#,
                get_hl_type_with_null(ty),
                name.to_lower_camel_case()
            )?;
        }

        if has_return_type {
            write!(
                writer,
                r#"
     * @return {{{}}}"#,
                return_type_with_null
            )?;
        }

        write!(
            writer,
            r#"
     */"#
        )?;
    }

    write!(
        writer,
        r#"
    {}{}("#,
        if is_static { "static " } else { "" },
        method
    )?;

    for (i, (name, ty)) in function
        .inputs
        .iter()
        .skip(usize::from(!is_static))
        .enumerate()
    {
        if i != 0 {
            write!(writer, ", ")?;
        }
        write!(writer, "{}", name.to_lower_camel_case())?;
        if type_script {
            write!(writer, ": {}", get_hl_type_with_null(ty))?;
        }
    }

    if type_script && has_return_type {
        write!(
            writer,
            r#"): {} {{
        "#,
            return_type_with_null
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
                r#"if (ref.isNull({name}.ptr)) {{
            throw "{name} is disposed";
        }}
        "#,
                name = name.to_lower_camel_case()
            )?;
        }
    }

    if has_return_type {
        if function.output.is_custom {
            write!(
                writer,
                r#"const result = new {}("#,
                return_type_without_null
            )?;
        } else {
            write!(writer, "const result = ")?;
        }
    }

    write!(writer, r#"liveSplitCoreNative.{}("#, &function.name)?;

    for (i, (name, typ)) in function.inputs.iter().enumerate() {
        if i != 0 {
            write!(writer, ", ")?;
        }
        write!(
            writer,
            "{}",
            if name == "this" {
                "this.ptr".to_string()
            } else if typ.name == "Json" {
                format!("JSON.stringify({})", name.to_lower_camel_case())
            } else if typ.is_custom {
                format!("{}.ptr", name.to_lower_camel_case())
            } else {
                name.to_lower_camel_case()
            }
        )?;
    }

    write!(writer, ")")?;

    if has_return_type && function.output.is_custom {
        write!(writer, r#")"#)?;
    }

    write!(writer, r#";"#)?;

    for (name, typ) in function.inputs.iter() {
        if typ.is_custom && typ.kind == TypeKind::Value {
            write!(
                writer,
                r#"
        {}.ptr = ref.NULL;"#,
                name.to_lower_camel_case()
            )?;
        }
    }

    if has_return_type {
        if function.output.is_nullable && function.output.is_custom {
            write!(
                writer,
                r#"
        if (ref.isNull(result.ptr)) {{
            return null;
        }}"#
            )?;
        }
        if is_json {
            write!(
                writer,
                r#"
        return JSON.parse(result);"#
            )?;
        } else {
            write!(
                writer,
                r#"
        return result;"#
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

pub fn write<W: Write>(
    mut writer: W,
    classes: &BTreeMap<String, Class>,
    type_script: bool,
) -> Result<()> {
    if type_script {
        write!(
            writer,
            r#""use strict";
// tslint:disable
import ffi = require('ffi');
import fs = require('fs');
import ref = require('ref');

{}

const liveSplitCoreNative = ffi.Library('livesplit_core', {{"#,
            typescript::HEADER
        )?;
    } else {
        write!(
            writer,
            "{}",
            r#""use strict";
const ffi = require('ffi');
const fs = require('fs');
const ref = require('ref');

const liveSplitCoreNative = ffi.Library('livesplit_core', {"#
        )?;
    }

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
    '{}': [{}, ["#,
                function.name,
                get_ll_type(&function.output)
            )?;

            for (i, (_, typ)) in function.inputs.iter().enumerate() {
                if i != 0 {
                    write!(writer, ", ")?;
                }
                write!(writer, "{}", get_ll_type(typ))?;
            }

            write!(writer, "]],")?;
        }
    }

    writeln!(
        writer,
        "{}",
        r#"
});"#
    )?;

    for (class_name, class) in classes {
        let class_name_ref = format!("{}Ref", class_name);
        let class_name_ref_mut = format!("{}RefMut", class_name);

        write_class_comments(&mut writer, &class.comments)?;

        write!(
            writer,
            r#"
{export}class {class} {{"#,
            class = class_name_ref,
            export = if type_script { "export " } else { "" }
        )?;

        if type_script {
            write!(
                writer,
                r#"
    ptr: Buffer;"#
            )?;
        }

        for function in &class.shared_fns {
            write_fn(&mut writer, function, type_script)?;
        }

        if class_name == "SharedTimer" {
            if type_script {
                write!(
                    writer,
                    "{}",
                    r#"
    readWith<T>(action: (timer: TimerRef) => T): T {
        return this.read().with(function (lock) {
            return action(lock.timer());
        });
    }
    writeWith<T>(action: (timer: TimerRefMut) => T): T {
        return this.write().with(function (lock) {
            return action(lock.timer());
        });
    }"#
                )?;
            } else {
                write!(
                    writer,
                    "{}",
                    r#"
    /**
     * @param {function(TimerRef)} action
     */
    readWith(action) {
        return this.read().with(function (lock) {
            return action(lock.timer());
        });
    }
    /**
     * @param {function(TimerRefMut)} action
     */
    writeWith(action) {
        return this.write().with(function (lock) {
            return action(lock.timer());
        });
    }"#
                )?;
            }
        }

        if type_script {
            write!(
                writer,
                r#"
    constructor(ptr: Buffer) {{"#
            )?;
        } else {
            write!(
                writer,
                r#"
    /**
     * @param {{Buffer}} ptr
     */
    constructor(ptr) {{"#
            )?;
        }

        write!(
            writer,
            r#"
        this.ptr = ptr;
    }}
}}
"#
        )?;

        if !type_script {
            writeln!(
                writer,
                r#"exports.{base_class} = {base_class};"#,
                base_class = class_name_ref
            )?;
        }

        write_class_comments(&mut writer, &class.comments)?;

        write!(
            writer,
            r#"
{export}class {class} extends {base_class} {{"#,
            class = class_name_ref_mut,
            base_class = class_name_ref,
            export = if type_script { "export " } else { "" }
        )?;

        for function in &class.mut_fns {
            write_fn(&mut writer, function, type_script)?;
        }

        if class_name == "RunEditor" {
            if type_script {
                write!(
                    writer,
                    "{}",
                    r#"
    setGameIconFromArray(data: Int8Array) {
        let buf = Buffer.from(data.buffer);
        if (data.byteLength !== data.buffer.byteLength) {
            buf = buf.slice(data.byteOffset, data.byteOffset + data.byteLength);
        }
        this.setGameIcon(buf, buf.byteLength);
    }
    activeSetIconFromArray(data: Int8Array) {
        let buf = Buffer.from(data.buffer);
        if (data.byteLength !== data.buffer.byteLength) {
            buf = buf.slice(data.byteOffset, data.byteOffset + data.byteLength);
        }
        this.activeSetIconFromArray(buf, buf.byteLength);
    }"#
                )?;
            } else {
                write!(
                    writer,
                    "{}",
                    r#"
    /**
     * @param {Int8Array} data
     */
    setGameIconFromArray(data) {
        let buf = Buffer.from(data.buffer);
        if (data.byteLength !== data.buffer.byteLength) {
            buf = buf.slice(data.byteOffset, data.byteOffset + data.byteLength);
        }
        this.setGameIcon(buf, buf.byteLength);
    }
    /**
     * @param {Int8Array} data
     */
    activeSetIconFromArray(data) {
        let buf = Buffer.from(data.buffer);
        if (data.byteLength !== data.buffer.byteLength) {
            buf = buf.slice(data.byteOffset, data.byteOffset + data.byteLength);
        }
        this.activeSetIconFromArray(buf, buf.byteLength);
    }"#
                )?;
            }
        }

        write!(
            writer,
            r#"
}}
"#
        )?;

        if !type_script {
            writeln!(
                writer,
                r#"exports.{base_class} = {base_class};"#,
                base_class = class_name_ref_mut
            )?;
        }

        write_class_comments(&mut writer, &class.comments)?;

        write!(
            writer,
            r#"
{export}class {class} extends {base_class} {{"#,
            class = class_name,
            base_class = class_name_ref_mut,
            export = if type_script { "export " } else { "" }
        )?;

        if type_script {
            write!(
                writer,
                r#"
    with<T>(closure: (obj: {class}) => T): T {{"#,
                class = class_name
            )?;
        } else {
            write!(
                writer,
                r#"
    /**
     * @param {{function({class})}} closure
     */
    with(closure) {{"#,
                class = class_name
            )?;
        }

        write!(
            writer,
            r#"
        try {{
            return closure(this);
        }} finally {{
            this.dispose();
        }}
    }}
    dispose() {{
        if (!ref.isNull(this.ptr)) {{"#
        )?;

        if let Some(function) = class.own_fns.iter().find(|f| f.method == "drop") {
            write!(
                writer,
                r#"
            liveSplitCoreNative.{}(this.ptr);"#,
                function.name
            )?;
        }

        write!(
            writer,
            r#"
            this.ptr = ref.NULL;
        }}
    }}"#
        )?;

        for function in class.static_fns.iter().chain(class.own_fns.iter()) {
            if function.method != "drop" {
                write_fn(&mut writer, function, type_script)?;
            }
        }

        if class_name == "Run" {
            if type_script {
                write!(
                    writer,
                    "{}",
                    r#"
    static parseArray(data: Int8Array, loadFilesPath: string): ParseRunResult {
        let buf = Buffer.from(data.buffer);
        if (data.byteLength !== data.buffer.byteLength) {
            buf = buf.slice(data.byteOffset, data.byteOffset + data.byteLength);
        }
        return Run.parse(buf, buf.byteLength, loadFilesPath);
    }
    static parseFile(file: any, loadFilesPath: string): ParseRunResult {
        const data = fs.readFileSync(file);
        return Run.parse(data, data.byteLength, loadFilesPath);
    }
    static parseString(text: string, loadFilesPath: string): ParseRunResult {
        const data = new Buffer(text);
        return Run.parse(data, data.byteLength, loadFilesPath);
    }"#
                )?;
            } else {
                write!(
                    writer,
                    "{}",
                    r#"
    /**
     * @param {Int8Array} data
     * @param {string} loadFilesPath
     * @return {ParseRunResult}
     */
    static parseArray(data, loadFilesPath) {
        let buf = Buffer.from(data.buffer);
        if (data.byteLength !== data.buffer.byteLength) {
            buf = buf.slice(data.byteOffset, data.byteOffset + data.byteLength);
        }
        return Run.parse(buf, buf.byteLength, loadFilesPath);
    }
    /**
     * @param {string | Buffer | number} file
     * @param {string} loadFilesPath
     * @return {ParseRunResult}
     */
    static parseFile(file, loadFilesPath) {
        const data = fs.readFileSync(file);
        return Run.parse(data, data.byteLength, loadFilesPath);
    }
    /**
     * @param {string} text
     * @param {string} loadFilesPath
     * @return {ParseRunResult}
     */
    static parseString(text, loadFilesPath) {
        const data = new Buffer(text);
        return Run.parse(data, data.byteLength, loadFilesPath);
    }"#
                )?;
            }
        } else if class_name == "Layout" {
            if type_script {
                write!(
                    writer,
                    "{}",
                    r#"
    static parseOriginalLivesplitArray(data: Int8Array): Layout | null {
        let buf = Buffer.from(data.buffer);
        if (data.byteLength !== data.buffer.byteLength) {
            buf = buf.slice(data.byteOffset, data.byteOffset + data.byteLength);
        }
        return Layout.parseOriginalLivesplit(buf, buf.byteLength);
    }
    static parseOriginalLivesplitString(text: string): Layout | null {
        const data = new Buffer(text);
        return Layout.parseOriginalLivesplit(data, data.byteLength);
    }"#
                )?;
            } else {
                write!(
                    writer,
                    "{}",
                    r#"
    /**
     * @param {Int8Array} data
     * @return {Layout | null}
     */
    static parseOriginalLivesplitArray(data) {
        let buf = Buffer.from(data.buffer);
        if (data.byteLength !== data.buffer.byteLength) {
            buf = buf.slice(data.byteOffset, data.byteOffset + data.byteLength);
        }
        return Layout.parseOriginalLivesplit(buf, buf.byteLength);
    }
    /**
     * @param {string} text
     * @return {Layout | null}
     */
    static parseOriginalLivesplitString(text) {
        const data = new Buffer(text);
        return Layout.parseOriginalLivesplit(data, data.byteLength);
    }"#
                )?;
            }
        }

        writeln!(
            writer,
            r#"
}}{export}"#,
            export = if type_script {
                "".to_string()
            } else {
                format!(
                    r#"
exports.{base_class} = {base_class};"#,
                    base_class = class_name
                )
            }
        )?;
    }

    Ok(())
}
