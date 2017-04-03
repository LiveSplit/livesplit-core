use std::io::{Write, Result};
use {Function, Type, TypeKind};
use heck::{CamelCase, MixedCase};

fn get_type(ty: &Type, output: bool) -> &str {
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
                "bool" => "bool",
                "()" => "void",
                "c_char" => "char",
                x => x,
            }
        }
        _ => "IntPtr",
    }
}

pub fn write<W: Write>(mut writer: W, functions: &[Function]) -> Result<()> {
    let mut prefix = String::from("");

    write!(writer,
           "{}",
           r#"using System;
using System.Runtime.InteropServices;
using System.Text;
using System.IO;

namespace LiveSplitCore
{"#)?;

    for function in functions {
        let name = function.name.to_string();
        let mut splits = name.splitn(2, '_');
        let new_prefix = splits.next().unwrap();
        let postfix = splits.next().unwrap();
        if new_prefix != prefix {
            if !prefix.is_empty() {
                if prefix == "Run" {
                    write!(writer,
                           "{}",
                           r#"
        public static IntPtr Parse(FileStream file)
        {
            var data = new byte[file.Length];
            file.Read(data, 0, data.Length);
            IntPtr pnt = Marshal.AllocHGlobal(data.Length);
            try
            {
                Marshal.Copy(data, 0, pnt, data.Length);
                return Parse(pnt, (UIntPtr)data.Length);
            }
            finally
            {
                Marshal.FreeHGlobal(pnt);
            }
        }"#)?;
                }
                writeln!(writer,
                         "{}",
                         r#"
    }"#)?;
            }
            write!(writer,
                   r#"
    public static class {}
    {{"#,
                   new_prefix)?;
        }
        prefix = new_prefix.to_string();

        write!(writer,
               r#"
        [DllImport("livesplit_core", EntryPoint="{}")]
        public static extern {} {}("#,
               &function.name,
               get_type(&function.output, true),
               postfix.to_camel_case())?;

        for (i, &(ref name, ref typ)) in function.inputs.iter().enumerate() {
            if i != 0 {
                write!(writer, ", ")?;
            }
            write!(writer,
                   "{} {}",
                   get_type(typ, false),
                   if name == "this" {
                       String::from("self")
                   } else {
                       name.to_mixed_case()
                   })?;
        }

        write!(writer, ");")?;
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
