use std::io::{Write, Result};
use {Class, Function, Type, TypeKind};
use heck::{CamelCase, MixedCase};
use std::collections::BTreeMap;

fn get_hl_type(ty: &Type) -> String {
    if ty.is_custom {
        match ty.kind {
            TypeKind::Ref => format!("{}", ty.name),
            TypeKind::RefMut => format!("{}", ty.name),
            TypeKind::Value => ty.name.clone(),
        }
    } else if ty.name == "bool" {
        "bool".to_string()
    } else if ty.name == "usize" {
        "long".to_string()
    } else {
        get_ll_type(ty, false).to_string()
    }
}

fn get_ll_type(ty: &Type, output: bool) -> &str {
    match (ty.kind, ty.name.as_str()) {
        (TypeKind::Ref, "c_char") => if output { "LSCoreString" } else { "string" },
        (TypeKind::Ref, _) |
        (TypeKind::RefMut, _) => "IntPtr",
        (_, t) if !ty.is_custom => {
            match t {
                "i8" => "sbyte",
                "i16" => "short",
                "i32" => "int",
                "i64" => "long",
                "u8" => "byte",
                "u16" => "ushort",
                "u32" => "uint",
                "u64" => "ulong",
                "usize" => "UIntPtr",
                "f32" => "float",
                "f64" => "double",
                "bool" => if output { "byte" } else { "bool" },
                "()" => "void",
                "c_char" => "char",
                x => x,
            }
        }
        _ => "IntPtr",
    }
}

pub fn write<W: Write>(mut writer: W, classes: &BTreeMap<String, Class>) -> Result<()> {
    write!(writer,
           "{}",
           r#"using System;
using System.Runtime.InteropServices;
using System.Text;
using System.IO;

namespace LiveSplitCore
{"#)?;

    for (class_name, class) in classes {
        write!(writer,
               r#"
    public class {class}
    {{
        internal IntPtr ptr;"#,
               class = class_name)?;

        if class_name == "Run" {
            write!(writer,
                   "{}",
                   r#"

        public static Run Parse(Stream stream)
        {
            var data = new byte[stream.Length];
            stream.Read(data, 0, data.Length);
            IntPtr pnt = Marshal.AllocHGlobal(data.Length);
            try
            {
                Marshal.Copy(data, 0, pnt, data.Length);
                return Parse(pnt, data.Length);
            }
            finally
            {
                Marshal.FreeHGlobal(pnt);
            }
        }"#)?;
        } else if class_name == "SharedTimer" {
            write!(writer,
                   "{}",
                   r#"

        public void ReadWith(Action<Timer> action)
        {
            var timerLock = Read();
            action(timerLock.Timer());
            timerLock.Drop();
        }

        public void WriteWith(Action<Timer> action)
        {
            var timerLock = Write();
            action(timerLock.Timer());
            timerLock.Drop();
        }"#)?;
        }

        for function in class.static_fns
            .iter()
            .chain(class.own_fns.iter())
            .chain(class.shared_fns.iter())
            .chain(class.mut_fns.iter()) {
            let is_static = function.is_static();
            let has_return_type = function.has_return_type();
            let return_type = get_hl_type(&function.output);
            let return_type_ll = get_ll_type(&function.output, true);
            let is_constructor = function.method == "new";

            if is_constructor {
                write!(writer,
                       r#"

        public {}("#,
                       class_name)?;
            } else {
                write!(writer,
                       r#"

        public{} {} {}("#,
                       if is_static { " static" } else { "" },
                       return_type,
                       function.method.to_camel_case())?;
            }

            for (i, &(ref name, ref typ)) in
                function.inputs.iter().skip(if is_static { 0 } else { 1 }).enumerate() {
                if i != 0 {
                    write!(writer, ", ")?;
                }
                write!(writer, "{} {}", get_hl_type(typ), name.to_mixed_case())?;
            }

            write!(writer,
                   r#")
        {{
            "#)?;

            if has_return_type {
                if is_constructor {
                    write!(writer, "this.ptr = ")?;
                } else if function.output.is_custom {
                    write!(writer, r#"return new {}("#, return_type)?;
                } else {
                    write!(writer, "return ")?;
                    if return_type_ll == "UIntPtr" {
                        write!(writer, "(long)")?;
                    }
                }
            }

            write!(writer, r#"LiveSplitCoreNative.{}("#, &function.name)?;

            for (i, &(ref name, ref typ)) in function.inputs.iter().enumerate() {
                if i != 0 {
                    write!(writer, ", ")?;
                }
                let ty_name = get_ll_type(typ, false);
                write!(writer,
                       "{}",
                       if name == "this" {
                           "this.ptr".to_string()
                       } else if ty_name == "UIntPtr" {
                           format!("(UIntPtr){}", name.to_mixed_case())
                       } else if typ.is_custom {
                           format!("{}.ptr", name.to_mixed_case())
                       } else {
                           name.to_mixed_case()
                       })?;
            }

            write!(writer,
                   "){}",
                   if return_type == "string" {
                       ".AsString()"
                   } else if return_type == "bool" {
                       " != 0"
                   } else {
                       ""
                   })?;

            if !is_constructor && has_return_type && function.output.is_custom {
                write!(writer, r#")"#)?;
            }

            write!(writer,
                   r#";
        }}"#)?;
        }

        writeln!(writer,
                 r#"

        internal {class}(IntPtr ptr)
        {{
            this.ptr = ptr;
        }}
    }}"#,
                 class = class_name)?;
    }

    write!(writer,
           r#"
    public static class LiveSplitCoreNative
    {{"#)?;

    for class in classes.values() {
        for function in class.static_fns
            .iter()
            .chain(class.own_fns.iter())
            .chain(class.shared_fns.iter())
            .chain(class.mut_fns.iter()) {
            write!(writer,
                   r#"
        [DllImport("livesplit_core")]
        public static extern {} {}("#,
                   get_ll_type(&function.output, true),
                   &function.name)?;

            for (i, &(ref name, ref typ)) in function.inputs.iter().enumerate() {
                if i != 0 {
                    write!(writer, ", ")?;
                }
                write!(writer,
                       "{} {}",
                       get_ll_type(typ, false),
                       if name == "this" {
                           String::from("self")
                       } else {
                           name.clone()
                       })?;
            }

            write!(writer, ");")?;
        }
    }

    writeln!(writer,
             "{}",
             r#"
    }

    public class LSCoreString : SafeHandle
    {
        public LSCoreString() : base(IntPtr.Zero, false) { }

        public override bool IsInvalid
        {
            get { return false; }
        }

        public string AsString()
        {
            int len = 0;
            while (Marshal.ReadByte(handle, len) != 0) { ++len; }
            byte[] buffer = new byte[len];
            Marshal.Copy(handle, buffer, 0, buffer.Length);
            return Encoding.UTF8.GetString(buffer);
        }

        protected override bool ReleaseHandle()
        {
            return true;
        }
    }
}"#)
}
