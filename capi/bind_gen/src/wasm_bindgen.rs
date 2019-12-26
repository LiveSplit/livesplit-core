use crate::{typescript, Class, Function, Type, TypeKind};
use heck::MixedCase;
use std::collections::BTreeMap;
use std::io::{Result, Write};

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
    let method = function.method.to_mixed_case();
    let is_json = has_return_type && function.output.name == "Json";

    if function
        .inputs
        .iter()
        .any(|(_, ty)| match (ty.kind, ty.is_custom, &ty.name) {
            (TypeKind::Value, false, n) if n == "i64" || n == "u64" => true,
            _ => false,
        })
    {
        return Ok(());
    }

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
        for (name, ty) in function.inputs.iter().skip(if is_static { 0 } else { 1 }) {
            write!(
                writer,
                r#"
     * @param {{{}}} {}"#,
                get_hl_type_with_null(ty),
                name.to_mixed_case()
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
        .skip(if is_static { 0 } else { 1 })
        .enumerate()
    {
        if i != 0 {
            write!(writer, ", ")?;
        }
        write!(writer, "{}", name.to_mixed_case())?;
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
                r#"if ({name}.ptr == 0) {{
            throw "{name} is disposed";
        }}
        "#,
                name = name.to_mixed_case()
            )?;
        }
    }

    for (name, typ) in function.inputs.iter() {
        let hl_type = get_hl_type_without_null(typ);
        if hl_type == "string" {
            write!(
                writer,
                r#"const {0}_allocated = allocString({0});
        "#,
                name.to_mixed_case()
            )?;
        } else if typ.name == "Json" {
            write!(
                writer,
                r#"const {0}_allocated = allocString(JSON.stringify({0}));
        "#,
                name.to_mixed_case()
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

    write!(writer, r#"wasm.{}("#, &function.name)?;

    for (i, (name, typ)) in function.inputs.iter().enumerate() {
        let type_name = get_hl_type_without_null(typ);
        if i != 0 {
            write!(writer, ", ")?;
        }
        write!(
            writer,
            "{}",
            if name == "this" {
                "this.ptr".to_string()
            } else if type_name == "string" || typ.name == "Json" {
                format!("{}_allocated.ptr", name.to_mixed_case())
            } else if typ.is_custom {
                format!("{}.ptr", name.to_mixed_case())
            } else {
                name.to_mixed_case()
            }
        )?;
        if type_name == "boolean" {
            write!(writer, " ? 1 : 0")?;
        }
    }

    write!(
        writer,
        "){}",
        if return_type_without_null == "boolean" {
            " != 0"
        } else {
            ""
        }
    )?;

    if has_return_type && function.output.is_custom {
        write!(writer, r#")"#)?;
    }

    write!(writer, r#";"#)?;

    for (name, typ) in function.inputs.iter() {
        let hl_type = get_hl_type_without_null(typ);
        if hl_type == "string" || typ.name == "Json" {
            write!(
                writer,
                r#"
        dealloc({}_allocated);"#,
                name.to_mixed_case()
            )?;
        }
    }

    for (name, typ) in function.inputs.iter() {
        if typ.is_custom && typ.kind == TypeKind::Value {
            write!(
                writer,
                r#"
        {}.ptr = 0;"#,
                name.to_mixed_case()
            )?;
        }
    }

    if has_return_type {
        if function.output.is_nullable {
            if function.output.is_custom {
                write!(
                    writer,
                    r#"
        if (result.ptr == 0) {{
            return null;
        }}"#
                )?;
            } else {
                write!(
                    writer,
                    r#"
        if (result == 0) {{
            return null;
        }}"#
                )?;
            }
        }
        if is_json {
            write!(
                writer,
                r#"
        return JSON.parse(decodeString(result));"#
            )?;
        } else if return_type_without_null == "string" {
            write!(
                writer,
                r#"
        return decodeString(result);"#
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
        writeln!(
            writer,
            "{}{}",
            r#"// tslint:disable
import * as wasm from "./livesplit_core_bg";

declare class TextEncoder {
    constructor(label?: string, options?: TextEncoding.TextEncoderOptions);
    encoding: string;
    encode(input?: string, options?: TextEncoding.TextEncodeOptions): Uint8Array;
}

declare class TextDecoder {
    constructor(utfLabel?: string, options?: TextEncoding.TextDecoderOptions)
    encoding: string;
    fatal: boolean;
    ignoreBOM: boolean;
    decode(input?: ArrayBufferView, options?: TextEncoding.TextDecodeOptions): string;
}

declare namespace TextEncoding {
    interface TextDecoderOptions {
        fatal?: boolean;
        ignoreBOM?: boolean;
    }

    interface TextDecodeOptions {
        stream?: boolean;
    }

    interface TextEncoderOptions {
        NONSTANDARD_allowLegacyEncoding?: boolean;
    }

    interface TextEncodeOptions {
        stream?: boolean;
    }

    interface TextEncodingStatic {
        TextDecoder: typeof TextDecoder;
        TextEncoder: typeof TextEncoder;
    }
}

let encodeUtf8: (str: string) => Uint8Array;
if (!(global as any)["TextEncoder"]) {
    encodeUtf8 = (str) => {
        var utf8 = [];
        for (var i = 0; i < str.length; i++) {
            var charcode = str.charCodeAt(i);
            if (charcode < 0x80) {
                utf8.push(charcode);
            } else if (charcode < 0x800) {
                utf8.push(0xc0 | (charcode >> 6),
                    0x80 | (charcode & 0x3f));
            } else if (charcode < 0xd800 || charcode >= 0xe000) {
                utf8.push(0xe0 | (charcode >> 12),
                    0x80 | ((charcode >> 6) & 0x3f),
                    0x80 | (charcode & 0x3f));
            } else {
                i++;
                charcode = 0x10000 + (((charcode & 0x3ff) << 10)
                    | (str.charCodeAt(i) & 0x3ff))
                utf8.push(0xf0 | (charcode >> 18),
                    0x80 | ((charcode >> 12) & 0x3f),
                    0x80 | ((charcode >> 6) & 0x3f),
                    0x80 | (charcode & 0x3f));
            }
        }
        return new Uint8Array(utf8);
    };
} else {
    const encoder = new TextEncoder("UTF-8");
    encodeUtf8 = (str) => encoder.encode(str);
}

let decodeUtf8: (data: Uint8Array) => string;
if (!(global as any)["TextDecoder"]) {
    decodeUtf8 = (data) => {
        var str = '',
            i;

        for (i = 0; i < data.length; i++) {
            var value = data[i];

            if (value < 0x80) {
                str += String.fromCharCode(value);
            } else if (value > 0xBF && value < 0xE0) {
                str += String.fromCharCode((value & 0x1F) << 6 | data[i + 1] & 0x3F);
                i += 1;
            } else if (value > 0xDF && value < 0xF0) {
                str += String.fromCharCode((value & 0x0F) << 12 | (data[i + 1] & 0x3F) << 6 | data[i + 2] & 0x3F);
                i += 2;
            } else {
                var charCode = ((value & 0x07) << 18 | (data[i + 1] & 0x3F) << 12 | (data[i + 2] & 0x3F) << 6 | data[i + 3] & 0x3F) - 0x010000;

                str += String.fromCharCode(charCode >> 10 | 0xD800, charCode & 0x03FF | 0xDC00);
                i += 3;
            }
        }

        return str;
    };
} else {
    const decoder = new TextDecoder("UTF-8");
    decodeUtf8 = (data) => decoder.decode(data);
}

interface Slice {
    ptr: number,
    len: number,
}

function allocInt8Array(src: Int8Array): Slice {
    const len = src.length;
    const ptr = wasm.alloc(len);
    const slice = new Uint8Array(wasm.memory.buffer, ptr, len);

    slice.set(src);

    return { ptr, len };
}

function allocString(str: string): Slice {
    const stringBuffer = encodeUtf8(str);
    const len = stringBuffer.length + 1;
    const ptr = wasm.alloc(len);
    const slice = new Uint8Array(wasm.memory.buffer, ptr, len);

    slice.set(stringBuffer);
    slice[len - 1] = 0;

    return { ptr, len };
}

function decodeString(ptr: number): string {
    const memory = new Uint8Array(wasm.memory.buffer);
    let end = ptr;
    while (memory[end] !== 0) {
        end += 1;
    }
    const slice = memory.slice(ptr, end);
    return decodeUtf8(slice);
}

function dealloc(slice: Slice) {
    wasm.dealloc(slice.ptr, slice.len);
}

"#,
            typescript::HEADER,
        )?;
    } else {
        writeln!(
            writer,
            "{}",
            r#"import * as wasm from "./livesplit_core_bg.wasm";

let encodeUtf8;
if (!global["TextEncoder"]) {
    encodeUtf8 = (str) => {
        var utf8 = [];
        for (var i = 0; i < str.length; i++) {
            var charcode = str.charCodeAt(i);
            if (charcode < 0x80) {
                utf8.push(charcode);
            } else if (charcode < 0x800) {
                utf8.push(0xc0 | (charcode >> 6),
                    0x80 | (charcode & 0x3f));
            }
            else if (charcode < 0xd800 || charcode >= 0xe000) {
                utf8.push(0xe0 | (charcode >> 12),
                    0x80 | ((charcode >> 6) & 0x3f),
                    0x80 | (charcode & 0x3f));
            } else {
                i++;
                charcode = 0x10000 + (((charcode & 0x3ff) << 10)
                    | (str.charCodeAt(i) & 0x3ff))
                utf8.push(0xf0 | (charcode >> 18),
                    0x80 | ((charcode >> 12) & 0x3f),
                    0x80 | ((charcode >> 6) & 0x3f),
                    0x80 | (charcode & 0x3f));
            }
        }
        return new Uint8Array(utf8);
    };
} else {
    const encoder = new TextEncoder("UTF-8");
    encodeUtf8 = (str) => encoder.encode(str);
}

let decodeUtf8;
if (!global["TextDecoder"]) {
    decodeUtf8 = (data) => {
        var str = '',
            i;

        for (i = 0; i < data.length; i++) {
            var value = data[i];

            if (value < 0x80) {
                str += String.fromCharCode(value);
            } else if (value > 0xBF && value < 0xE0) {
                str += String.fromCharCode((value & 0x1F) << 6 | data[i + 1] & 0x3F);
                i += 1;
            } else if (value > 0xDF && value < 0xF0) {
                str += String.fromCharCode((value & 0x0F) << 12 | (data[i + 1] & 0x3F) << 6 | data[i + 2] & 0x3F);
                i += 2;
            } else {
                var charCode = ((value & 0x07) << 18 | (data[i + 1] & 0x3F) << 12 | (data[i + 2] & 0x3F) << 6 | data[i + 3] & 0x3F) - 0x010000;

                str += String.fromCharCode(charCode >> 10 | 0xD800, charCode & 0x03FF | 0xDC00);
                i += 3;
            }
        }

        return str;
    };
} else {
    const decoder = new TextDecoder("UTF-8");
    decodeUtf8 = (data) => decoder.decode(data);
}

function allocInt8Array(src) {
    const len = src.length;
    const ptr = wasm.alloc(len);
    const slice = new Uint8Array(wasm.memory.buffer, ptr, len);

    slice.set(src);

    return { ptr, len };
}

function allocString(str) {
    const stringBuffer = encodeUtf8(str);
    const len = stringBuffer.length + 1;
    const ptr = wasm.alloc(len);
    const slice = new Uint8Array(wasm.memory.buffer, ptr, len);

    slice.set(stringBuffer);
    slice[len - 1] = 0;

    return { ptr, len };
}

function decodeString(ptr) {
    const memory = new Uint8Array(wasm.memory.buffer);
    let end = ptr;
    while (memory[end] !== 0) {
        end += 1;
    }
    const slice = memory.slice(ptr, end);
    return decodeUtf8(slice);
}

function dealloc(slice) {
    wasm.dealloc(slice.ptr, slice.len);
}"#,
        )?;
    }

    for (class_name, class) in classes {
        let class_name_ref = format!("{}Ref", class_name);
        let class_name_ref_mut = format!("{}RefMut", class_name);

        write_class_comments(&mut writer, &class.comments)?;

        write!(
            writer,
            r#"
export class {class} {{"#,
            class = class_name_ref,
        )?;

        if type_script {
            write!(
                writer,
                r#"
    ptr: number;"#
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
    /**
     * This constructor is an implementation detail. Do not use this.
     */
    constructor(ptr: number) {{"#
            )?;
        } else {
            write!(
                writer,
                r#"
    /**
     * This constructor is an implementation detail. Do not use this.
     * @param {{number}} ptr
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

        write_class_comments(&mut writer, &class.comments)?;

        write!(
            writer,
            r#"
export class {class} extends {base_class} {{"#,
            class = class_name_ref_mut,
            base_class = class_name_ref,
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
        const slice = allocInt8Array(data);
        this.setGameIcon(slice.ptr, slice.len);
        dealloc(slice);
    }
    activeSetIconFromArray(data: Int8Array) {
        const slice = allocInt8Array(data);
        this.activeSetIcon(slice.ptr, slice.len);
        dealloc(slice);
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
        const slice = allocInt8Array(data);
        this.setGameIcon(slice.ptr, slice.len);
        dealloc(slice);
    }
    /**
     * @param {Int8Array} data
     */
    activeSetIconFromArray(data) {
        const slice = allocInt8Array(data);
        this.activeSetIcon(slice.ptr, slice.len);
        dealloc(slice);
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

        write_class_comments(&mut writer, &class.comments)?;

        write!(
            writer,
            r#"
export class {class} extends {base_class} {{"#,
            class = class_name,
            base_class = class_name_ref_mut,
        )?;

        if type_script {
            write!(
                writer,
                r#"
    /**
     * Allows for scoped usage of the object. The object is guaranteed to get
     * disposed once this function returns. You are free to dispose the object
     * early yourself anywhere within the scope. The scope's return value gets
     * carried to the outside of this function.
     */
    with<T>(closure: (obj: {class}) => T): T {{"#,
                class = class_name
            )?;
        } else {
            write!(
                writer,
                r#"
    /**
     * Allows for scoped usage of the object. The object is guaranteed to get
     * disposed once this function returns. You are free to dispose the object
     * early yourself anywhere within the scope. The scope's return value gets
     * carried to the outside of this function.
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
    /**
     * Disposes the object, allowing it to clean up all of its memory. You need
     * to call this for every object that you don't use anymore and hasn't
     * already been disposed.
     */
    dispose() {{
        if (this.ptr != 0) {{"#
        )?;

        if let Some(function) = class.own_fns.iter().find(|f| f.method == "drop") {
            write!(
                writer,
                r#"
            wasm.{}(this.ptr);"#,
                function.name
            )?;
        }

        write!(
            writer,
            r#"
            this.ptr = 0;
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
    static parseArray(data: Int8Array, path: string, loadFiles: boolean): ParseRunResult {
        const slice = allocInt8Array(data);
        const result = Run.parse(slice.ptr, slice.len, path, loadFiles);
        dealloc(slice);
        return result;
    }
    static parseString(text: string, path: string, loadFiles: boolean): ParseRunResult {
        const slice = allocString(text);
        const result = Run.parse(slice.ptr, slice.len, path, loadFiles);
        dealloc(slice);
        return result;
    }"#
                )?;
            } else {
                write!(
                    writer,
                    "{}",
                    r#"
    /**
     * @param {Int8Array} data
     * @param {string} path
     * @param {boolean} loadFiles
     * @return {ParseRunResult}
     */
    static parseArray(data, path, loadFiles) {
        const slice = allocInt8Array(data);
        const result = Run.parse(slice.ptr, slice.len, path, loadFiles);
        dealloc(slice);
        return result;
    }
    /**
     * @param {string} text
     * @param {string} path
     * @param {boolean} loadFiles
     * @return {ParseRunResult}
     */
    static parseString(text, path, loadFiles) {
        const slice = allocString(text);
        const result = Run.parse(slice.ptr, slice.len, path, loadFiles);
        dealloc(slice);
        return result;
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
        const slice = allocInt8Array(data);
        const result = Layout.parseOriginalLivesplit(slice.ptr, slice.len);
        dealloc(slice);
        return result;
    }
    static parseOriginalLivesplitString(text: string): Layout | null {
        const slice = allocString(text);
        const result = Layout.parseOriginalLivesplit(slice.ptr, slice.len);
        dealloc(slice);
        return result;
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
        const slice = allocInt8Array(data);
        const result = Layout.parseOriginalLivesplit(slice.ptr, slice.len);
        dealloc(slice);
        return result;
    }
    /**
     * @param {string} text
     * @return {Layout | null}
     */
    static parseOriginalLivesplitString(text) {
        const slice = allocString(text);
        const result = Layout.parseOriginalLivesplit(slice.ptr, slice.len);
        dealloc(slice);
        return result;
    }"#
                )?;
            }
        }

        writeln!(
            writer,
            r#"
}}"#,
        )?;
    }

    Ok(())
}
