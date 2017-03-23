use std::io::{Write, Result};
use {Function, Type, TypeKind};

fn to_camel_case(snake_case: &str, cap_first_letter: bool) -> String {
    let mut camel_case = String::new();

    for (u, split) in snake_case.split('_').enumerate() {
        for (i, c) in split.char_indices() {
            if (cap_first_letter || u != 0) && i == 0 {
                camel_case.extend(c.to_uppercase());
            } else {
                camel_case.push(c);
            }
        }
    }

    camel_case
}

fn get_type(ty: &Type, output: bool) -> &str {
    match (ty.kind, ty.name.as_str()) {
        (TypeKind::Ref, "c_char") => if output { "LSCCoreString" } else { "string" },
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
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;

namespace LiveSplitCore
{
    public class LiveSplitCoreNative
    {"#)?;

    for function in functions {
        let name = function.name.to_string();
        let mut splits = name.splitn(2, '_');
        let new_prefix = splits.next().unwrap();
        let postfix = splits.next().unwrap();
        if new_prefix != prefix {
            if !prefix.is_empty() {
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
               to_camel_case(&postfix, true))?;

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
                       to_camel_case(name, false)
                   })?;
        }

        write!(writer, ");")?;
    }

    writeln!(writer,
             "{}",
             r#"
        }
    }

    internal class LSCoreString : SafeHandle
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
