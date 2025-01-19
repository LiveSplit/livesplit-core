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
            (_, t) if !ty.is_custom => match t {
                "i8" => "number",
                "i16" => "number",
                "i32" => "number",
                "i64" => "BigInt",
                "u8" => "number",
                "u16" => "number",
                "u32" => "number",
                "u64" => "BigInt",
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

    if function.inputs.iter().any(|(_, ty)| {
        matches!(
            (ty.kind, ty.is_custom, &*ty.name),
            (TypeKind::Value, false, "i64" | "u64")
        )
    }) {
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
     * @return {{{return_type_with_null}}}"#
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
            r#"): {return_type_with_null} {{
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
                r#"if ({name}.ptr == 0) {{
            throw "{name} is disposed";
        }}
        "#,
                name = name.to_lower_camel_case()
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
                name.to_lower_camel_case()
            )?;
        } else if typ.name == "Json" {
            write!(
                writer,
                r#"const {0}_allocated = allocString(JSON.stringify({0}));
        "#,
                name.to_lower_camel_case()
            )?;
        }
    }

    if has_return_type {
        if function.output.is_custom {
            write!(writer, r#"const result = new {return_type_without_null}("#)?;
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
                format!("{}_allocated.ptr", name.to_lower_camel_case())
            } else if typ.is_custom {
                format!("{}.ptr", name.to_lower_camel_case())
            } else {
                name.to_lower_camel_case()
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
                name.to_lower_camel_case()
            )?;
        }
    }

    for (name, typ) in function.inputs.iter() {
        if typ.is_custom && typ.kind == TypeKind::Value {
            write!(
                writer,
                r#"
        {}.ptr = 0;"#,
                name.to_lower_camel_case()
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

pub fn write_preload<W: Write>(mut writer: W, type_script: bool) -> Result<()> {
    let content = if type_script {
        r#"import init, { InitOutput } from "./livesplit_core.js";

let promise: Promise<InitOutput> | null = null;
export async function preloadWasm(): Promise<InitOutput> {
    if (!promise) {
        promise = init();
    }
    return await promise;
}"#
    } else {
        r#"import init from "./livesplit_core.js

let promise = null;
export async function preloadWasm() {
    if (!promise) {
        promise = init();
    }
    return await promise;
}"#
    };
    writeln!(writer, "{content}")
}

pub fn write<W: Write>(
    mut writer: W,
    classes: &BTreeMap<String, Class>,
    type_script: bool,
    bundler: bool,
) -> Result<()> {
    if type_script {
        if bundler {
            writeln!(
                writer,
                "{}",
                r#"// tslint:disable
import * as wasm from "./livesplit_core_bg.wasm";
import "./livesplit_core.js";"#
            )?;
        } else {
            writeln!(
                writer,
                "{}",
                r#"// tslint:disable
import { preloadWasm } from "./preload";

const wasm = await preloadWasm();"#
            )?;
        }
        writeln!(
            writer,
            "{}{}",
            r#"
const encoder = new TextEncoder();
const decoder = new TextDecoder();

interface Slice {
    ptr: number,
    len: number,
    cap: number,
}

function allocUint8Array(src: Uint8Array): Slice {
    const cap = src.length;
    const ptr = wasm.alloc(cap);
    const slice = new Uint8Array(wasm.memory.buffer, ptr, cap);

    slice.set(src);

    return { ptr, len: cap, cap };
}

function allocString(str: string): Slice {
    const cap = 3 * str.length + 1;
    const ptr = wasm.alloc(cap);
    const slice = new Uint8Array(wasm.memory.buffer, ptr, cap);

    const stats = encoder.encodeInto(str, slice);
    slice[stats.written] = 0;

    return { ptr, len: stats.written, cap };
}

function decodeSlice(ptr: number): Uint8Array {
    const memory = new Uint8Array(wasm.memory.buffer);
    const len = wasm.get_buf_len();
    return memory.slice(ptr, ptr + len);
}

function decodePtrLen(ptr: number, len: number): Uint8Array {
    const memory = new Uint8Array(wasm.memory.buffer);
    return memory.slice(ptr, ptr + len);
}

function decodeString(ptr: number): string {
    return decoder.decode(decodeSlice(ptr));
}

function dealloc(slice: Slice) {
    wasm.dealloc(slice.ptr, slice.cap);
}

"#,
            typescript::HEADER,
        )?;
    } else {
        if bundler {
            writeln!(
                writer,
                "{}",
                r#"import * as wasm from "./livesplit_core_bg.wasm";
import "./livesplit_core.js";"#
            )?;
        } else {
            writeln!(
                writer,
                "{}",
                r#"import { preloadWasm } from "./preload";

const wasm = await preloadWasm();"#
            )?;
        }
        writeln!(
            writer,
            "{}",
            r#"
const encoder = new TextEncoder();
const decoder = new TextDecoder();

function allocUint8Array(src) {
    const cap = src.length;
    const ptr = wasm.alloc(cap);
    const slice = new Uint8Array(wasm.memory.buffer, ptr, cap);

    slice.set(src);

    return { ptr, len: cap, cap };
}

function allocString(str) {
    const cap = 3 * str.length + 1;
    const ptr = wasm.alloc(cap);
    const slice = new Uint8Array(wasm.memory.buffer, ptr, cap);

    const stats = encoder.encodeInto(str, slice);
    slice[stats.written] = 0;

    return { ptr, len: stats.written, cap };
}

function decodeSlice(ptr) {
    const memory = new Uint8Array(wasm.memory.buffer);
    const len = wasm.get_buf_len();
    return memory.slice(ptr, ptr + len);
}

function decodePtrLen(ptr, len) {
    const memory = new Uint8Array(wasm.memory.buffer);
    return memory.slice(ptr, ptr + len);
}

function decodeString(ptr) {
    return decoder.decode(decodeSlice(ptr));
}

function dealloc(slice) {
    wasm.dealloc(slice.ptr, slice.cap);
}"#,
        )?;
    }

    for (class_name, class) in classes {
        let class_name_ref = format!("{class_name}Ref");
        let class_name_ref_mut = format!("{class_name}RefMut");

        write_class_comments(&mut writer, &class.comments)?;

        write!(
            writer,
            r#"
export class {class_name_ref} {{"#,
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
        using lock = this.read();
        return action(lock.timer());
    }
    writeWith<T>(action: (timer: TimerRefMut) => T): T {
        using lock = this.write();
        return action(lock.timer());
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
        using lock = this.read();
        return action(lock.timer());
    }
    /**
     * @param {function(TimerRefMut)} action
     */
    writeWith(action) {
        using lock = this.write();
        return action(lock.timer());
    }"#
                )?;
            }
        } else if class_name == "Timer" {
            if type_script {
                write!(
                    writer,
                    "{}",
                    r#"
    saveAsLssBytes(): Uint8Array {
        if (this.ptr == 0) {
            throw "this is disposed";
        }
        const result = wasm.Timer_save_as_lss(this.ptr);
        return decodeSlice(result);
    }"#
                )?;
            } else {
                write!(
                    writer,
                    "{}",
                    r#"
    /**
     * @return {Uint8Array}
     */
    saveAsLssBytes() {
        if (this.ptr == 0) {
            throw "this is disposed";
        }
        const result = wasm.Timer_save_as_lss(this.ptr);
        return decodeSlice(result);
    }"#
                )?;
            }
        } else if class_name == "Run" {
            if type_script {
                write!(
                    writer,
                    "{}",
                    r#"
    saveAsLssBytes(): Uint8Array {
        if (this.ptr == 0) {
            throw "this is disposed";
        }
        const result = wasm.Run_save_as_lss(this.ptr);
        return decodeSlice(result);
    }"#
                )?;
            } else {
                write!(
                    writer,
                    "{}",
                    r#"
    /**
     * @return {Uint8Array}
     */
    saveAsLssBytes() {
        if (this.ptr == 0) {
            throw "this is disposed";
        }
        const result = wasm.Run_save_as_lss(this.ptr);
        return decodeSlice(result);
    }"#
                )?;
            }
        } else if class_name == "ImageCache" {
            if type_script {
                write!(
                    writer,
                    "{}",
                    r#"
    /**
     * Looks up an image in the cache based on its image ID. The bytes are the image in its original
     * file format. The format is not specified and can be any image format. The
     * data may not even represent a valid image at all. If the image is not in the
     * cache, null is returned. This does not mark the image as visited.
     */
    lookupData(key: string): Uint8Array | undefined {
        if (this.ptr == 0) {
            throw "this is disposed";
        }
        const key_allocated = allocString(key);
        const ptr = wasm.ImageCache_lookup_data_ptr(this.ptr, key_allocated.ptr);
        const len = wasm.ImageCache_lookup_data_len(this.ptr, key_allocated.ptr);
        dealloc(key_allocated);
        if (ptr === 0) {
            return undefined;
        }
        return decodePtrLen(ptr, len);
    }"#
                )?;
            } else {
                write!(
                    writer,
                    "{}",
                    r#"
    /**
     * Looks up an image in the cache based on its image ID. The bytes are the image in its original
     * file format. The format is not specified and can be any image format. The
     * data may not even represent a valid image at all. If the image is not in the
     * cache, null is returned. This does not mark the image as visited.
     *
     * @param {string} key
     * @return {Uint8Array | undefined}
     */
    lookupData(key) {
        if (this.ptr == 0) {
            throw "this is disposed";
        }
        const key_allocated = allocString(key);
        const ptr = wasm.ImageCache_lookup_data_ptr(this.ptr, key_allocated.ptr);
        const len = wasm.ImageCache_lookup_data_len(this.ptr, key_allocated.ptr);
        dealloc(key_allocated);
        if (ptr === 0) {
            return undefined;
        }
        return decodePtrLen(ptr, len);
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
export class {class_name_ref_mut} extends {class_name_ref} {{"#,
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
    setGameIconFromArray(data: Uint8Array) {
        const slice = allocUint8Array(data);
        this.setGameIcon(slice.ptr, slice.len);
        dealloc(slice);
    }
    activeSetIconFromArray(data: Uint8Array) {
        const slice = allocUint8Array(data);
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
     * @param {Uint8Array} data
     */
    setGameIconFromArray(data) {
        const slice = allocUint8Array(data);
        this.setGameIcon(slice.ptr, slice.len);
        dealloc(slice);
    }
    /**
     * @param {Uint8Array} data
     */
    activeSetIconFromArray(data) {
        const slice = allocUint8Array(data);
        this.activeSetIcon(slice.ptr, slice.len);
        dealloc(slice);
    }"#
                )?;
            }
        } else if class_name == "ImageCache" {
            if type_script {
                write!(
                    writer,
                    "{}",
                    r#"
    cacheFromArray(data: Uint8Array, isLarge: boolean): string {
        const slice = allocUint8Array(data);
        const result = this.cache(slice.ptr, slice.len, isLarge);
        dealloc(slice);
        return result;
    }"#
                )?;
            } else {
                write!(
                    writer,
                    "{}",
                    r#"
    cacheFromArray(data, isLarge) {
        const slice = allocUint8Array(data);
        const result = this.cache(slice.ptr, slice.len, isLarge);
        dealloc(slice);
        return result;
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
export class {class_name} extends {class_name_ref_mut} {{
    /**
     * Disposes the object, allowing it to clean up all of its memory. You need
     * to call this for every object that you don't use anymore and hasn't
     * already been disposed.
     */
    [Symbol.dispose]() {{
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
    static parseArray(data: Uint8Array, loadFilesPath: string): ParseRunResult {
        const slice = allocUint8Array(data);
        const result = Run.parse(slice.ptr, slice.len, loadFilesPath);
        dealloc(slice);
        return result;
    }
    static parseString(text: string, loadFilesPath: string): ParseRunResult {
        const slice = allocString(text);
        const result = Run.parse(slice.ptr, slice.len, loadFilesPath);
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
     * @param {Uint8Array} data
     * @param {string} loadFilesPath
     * @return {ParseRunResult}
     */
    static parseArray(data, loadFilesPath) {
        const slice = allocUint8Array(data);
        const result = Run.parse(slice.ptr, slice.len, loadFilesPath);
        dealloc(slice);
        return result;
    }
    /**
     * @param {string} text
     * @param {string} loadFilesPath
     * @return {ParseRunResult}
     */
    static parseString(text, loadFilesPath) {
        const slice = allocString(text);
        const result = Run.parse(slice.ptr, slice.len, loadFilesPath);
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
    static parseOriginalLivesplitArray(data: Uint8Array): Layout | null {
        const slice = allocUint8Array(data);
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
     * @param {Uint8Array} data
     * @return {Layout | null}
     */
    static parseOriginalLivesplitArray(data) {
        const slice = allocUint8Array(data);
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
