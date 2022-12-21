use crate::{Class, Function, Type, TypeKind};
use heck::{ToLowerCamelCase, ToUpperCamelCase};
use std::{
    collections::BTreeMap,
    io::{Result, Write},
};

fn get_hl_type(ty: &Type) -> String {
    if ty.is_custom {
        match ty.kind {
            TypeKind::Ref => format!("{}Ref", ty.name),
            TypeKind::RefMut => format!("{}RefMut", ty.name),
            TypeKind::Value => ty.name.clone(),
        }
    } else if ty.name == "bool" {
        "bool".to_string()
    } else if ty.name == "usize" {
        "ulong".to_string()
    } else if ty.name == "isize" {
        "long".to_string()
    } else {
        let ret = get_ll_type(ty, false).to_string();
        if ret == "LSCoreString" {
            "string".to_string()
        } else {
            ret
        }
    }
}

fn get_ll_type(ty: &Type, output: bool) -> &str {
    match (ty.kind, ty.name.as_str()) {
        (TypeKind::Ref, "c_char") => "LSCoreString",
        (TypeKind::Ref, _) | (TypeKind::RefMut, _) => "IntPtr",
        (_, t) if !ty.is_custom => match t {
            "i8" => "sbyte",
            "i16" => "short",
            "i32" => "int",
            "i64" => "long",
            "u8" => "byte",
            "u16" => "ushort",
            "u32" => "uint",
            "u64" => "ulong",
            "usize" => "UIntPtr",
            "isize" => "IntPtr",
            "f32" => "float",
            "f64" => "double",
            "bool" => {
                if output {
                    "byte"
                } else {
                    "bool"
                }
            }
            "()" => "void",
            "c_char" => "char",
            "Json" => "LSCoreString",
            x => x,
        },
        _ => "IntPtr",
    }
}

fn write_class_comments<W: Write>(mut writer: W, comments: &[String]) -> Result<()> {
    write!(
        writer,
        r#"
    /// <summary>"#
    )?;

    for comment in comments {
        write!(
            writer,
            r#"
    /// {}"#,
            comment
                .replace("<NULL>", "null")
                .replace("<TRUE>", "true")
                .replace("<FALSE>", "false")
        )?;
    }

    write!(
        writer,
        r#"
    /// </summary>"#
    )
}

fn write_fn<W: Write>(mut writer: W, function: &Function, class_name: &str) -> Result<()> {
    let is_static = function.is_static();
    let has_return_type = function.has_return_type();
    let return_type = get_hl_type(&function.output);
    let return_type_ll = get_ll_type(&function.output, true);
    let is_constructor = function.method == "new" && !function.output.is_nullable;

    if !function.comments.is_empty() {
        write!(
            writer,
            r#"
        /// <summary>"#
        )?;

        for comment in &function.comments {
            write!(
                writer,
                r#"
        /// {}"#,
                comment
                    .replace("<NULL>", "null")
                    .replace("<TRUE>", "true")
                    .replace("<FALSE>", "false")
            )?;
        }

        write!(
            writer,
            r#"
        /// </summary>"#
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
            function.method.to_upper_camel_case()
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
            r#") : base(IntPtr.Zero)
        {{
            "#
        )?;
    } else {
        write!(
            writer,
            r#")
        {{
            "#
        )?;
    }

    for (name, typ) in function.inputs.iter() {
        if typ.is_custom {
            write!(
                writer,
                r#"if ({name}.ptr == IntPtr.Zero)
            {{
                throw new ObjectDisposedException("{name}");
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
            write!(writer, r#"var result = new {}("#, return_type)?;
        } else {
            write!(writer, "var result = ")?;
            if return_type_ll == "UIntPtr" {
                write!(writer, "(ulong)")?;
            } else if return_type_ll == "IntPtr" {
                write!(writer, "(long)")?;
            }
        }
    }

    write!(writer, r#"LiveSplitCoreNative.{}("#, &function.name)?;

    for (i, (name, typ)) in function.inputs.iter().enumerate() {
        if i != 0 {
            write!(writer, ", ")?;
        }
        let ty_name = get_ll_type(typ, false);
        write!(
            writer,
            "{}",
            if name == "this" {
                "this.ptr".to_string()
            } else if ty_name == "UIntPtr" {
                format!("(UIntPtr){}", name.to_lower_camel_case())
            } else if ty_name == "IntPtr" {
                format!("(IntPtr){}", name.to_lower_camel_case())
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
        if return_type == "bool" { " != 0" } else { "" }
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
            {}.ptr = IntPtr.Zero;"#,
                name.to_lower_camel_case()
            )?;
        }
    }

    if has_return_type && !is_constructor {
        if function.output.is_nullable && function.output.is_custom {
            write!(
                writer,
                r#"
            if (result.ptr == IntPtr.Zero)
            {{
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

pub fn write<W: Write>(mut writer: W, classes: &BTreeMap<String, Class>) -> Result<()> {
    write!(
        writer,
        "{}",
        r#"using System;
using System.Runtime.InteropServices;
using System.Text;
using System.IO;

namespace LiveSplitCore
{"#
    )?;

    for (class_name, class) in classes {
        let class_name_ref = format!("{}Ref", class_name);
        let class_name_ref_mut = format!("{}RefMut", class_name);

        write_class_comments(&mut writer, &class.comments)?;

        write!(
            writer,
            r#"
    public class {class}
    {{
        internal IntPtr ptr;"#,
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
        public void ReadWith(Action<TimerRef> action)
        {
            using (var timerLock = Read())
            {
                action(timerLock.Timer());
            }
        }
        public void WriteWith(Action<TimerRefMut> action)
        {
            using (var timerLock = Write())
            {
                action(timerLock.Timer());
            }
        }"#
            )?;
        }

        write!(
            writer,
            r#"
        internal {base_class}(IntPtr ptr)
        {{
            this.ptr = ptr;
        }}
    }}
"#,
            base_class = class_name_ref
        )?;

        write_class_comments(&mut writer, &class.comments)?;

        write!(
            writer,
            r#"
    public class {class} : {base_class}
    {{"#,
            class = class_name_ref_mut,
            base_class = class_name_ref
        )?;

        for function in &class.mut_fns {
            write_fn(&mut writer, function, &class_name_ref_mut)?;
        }

        write!(
            writer,
            r#"
        internal {class}(IntPtr ptr) : base(ptr) {{ }}
    }}
"#,
            class = class_name_ref_mut
        )?;

        write_class_comments(&mut writer, &class.comments)?;

        write!(
            writer,
            r#"
    public class {class} : {base_class}, IDisposable
    {{
        private void Drop()
        {{
            if (ptr != IntPtr.Zero)
            {{"#,
            class = class_name,
            base_class = class_name_ref_mut
        )?;

        if let Some(function) = class.own_fns.iter().find(|f| f.method == "drop") {
            write!(
                writer,
                r#"
                LiveSplitCoreNative.{}(this.ptr);"#,
                function.name
            )?;
        }

        write!(
            writer,
            r#"
                ptr = IntPtr.Zero;
            }}
        }}
        ~{class}()
        {{
            Drop();
        }}
        public void Dispose()
        {{
            Drop();
            GC.SuppressFinalize(this);
        }}"#,
            class = class_name
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
        public static ParseRunResult Parse(Stream stream, string loadFilesPath)
        {
            var data = new byte[stream.Length];
            stream.Read(data, 0, data.Length);
            IntPtr pnt = Marshal.AllocHGlobal(data.Length);
            try
            {
                Marshal.Copy(data, 0, pnt, data.Length);
                return Parse(pnt, data.Length, loadFilesPath);
            }
            finally
            {
                Marshal.FreeHGlobal(pnt);
            }
        }"#
            )?;
        }

        writeln!(
            writer,
            r#"
        internal {class}(IntPtr ptr) : base(ptr) {{ }}
    }}"#,
            class = class_name
        )?;
    }

    write!(
        writer,
        r#"
    public static class LiveSplitCoreNative
    {{"#
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
        [DllImport("livesplit_core", CallingConvention = CallingConvention.Cdecl)]
        public static extern {} {}("#,
                get_ll_type(&function.output, true),
                &function.name
            )?;

            for (i, (name, typ)) in function.inputs.iter().enumerate() {
                if i != 0 {
                    write!(writer, ", ")?;
                }
                write!(
                    writer,
                    "{} {}",
                    get_ll_type(typ, false),
                    if name == "this" {
                        String::from("self")
                    } else {
                        name.clone()
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
        [DllImport("livesplit_core", CallingConvention = CallingConvention.Cdecl)]
        public static extern UIntPtr get_buf_len();
    }

    public class LSCoreString : SafeHandle
    {
        private bool needToFree;

        public LSCoreString() : base(IntPtr.Zero, false) { }

        public override bool IsInvalid
        {
            get { return false; }
        }

        public static implicit operator LSCoreString(string managedString)
        {
            LSCoreString lsCoreString = new LSCoreString();

            int len = Encoding.UTF8.GetByteCount(managedString);
            byte[] buffer = new byte[len + 1];
            Encoding.UTF8.GetBytes(managedString, 0, managedString.Length, buffer, 0);
            IntPtr nativeUtf8 = Marshal.AllocHGlobal(buffer.Length);
            Marshal.Copy(buffer, 0, nativeUtf8, buffer.Length);

            lsCoreString.SetHandle(nativeUtf8);
            lsCoreString.needToFree = true;
            return lsCoreString;
        }

        /// Unsafely assumes that the length can be retrieved from
        /// `get_buf_len`. This is only true for strings that have actually been
        /// retrieved from livesplit-core.
        public static implicit operator string(LSCoreString lSCoreString)
        {
            var handle = lSCoreString.handle;
            if (handle == IntPtr.Zero)
                return null;

            byte[] buffer = new byte[(long)LiveSplitCoreNative.get_buf_len()];
            Marshal.Copy(handle, buffer, 0, buffer.Length);
            return Encoding.UTF8.GetString(buffer);
        }

        protected override bool ReleaseHandle()
        {
            if (needToFree)
            {
                Marshal.FreeHGlobal(handle);
            }
            return true;
        }
    }
}"#
    )
}
