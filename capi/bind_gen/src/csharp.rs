use std::io::{Write, Result};
use Function;
use syntex_syntax::ast::TyKind;
use syntex_syntax::symbol::Symbol;

fn get_type(ty: &TyKind) -> &'static str {
    if let &TyKind::Ptr(ref ptr) = ty {
        if let &TyKind::Path(_, ref path) = &ptr.ty.node {
            if let Some(segment) = path.segments.last() {
                let name = segment.identifier.name;
                if name == "c_char" {
                    return "string";
                } else {
                    return "IntPtr";
                }
            }
        }
    } else if let &TyKind::Path(_, ref path) = ty {
        if let Some(segment) = path.segments.last() {
            let name = segment.identifier.name;
            if name == "i8" {
                return "sbyte";
            } else if name == "i16" {
                return "short";
            } else if name == "i32" {
                return "int";
            } else if name == "i64" {
                return "long";
            } else if name == "u8" {
                return "byte";
            } else if name == "u16" {
                return "ushort";
            } else if name == "u32" {
                return "uint";
            } else if name == "u64" {
                return "ulong";
            } else if name == "f32" {
                return "float";
            } else if name == "f64" {
                return "double";
            } else if name == "usize" {
                return "IntPtr";
            } else if name == "bool" {
                return "bool"; // Not entirely sure about this
            } else if name == "TimingMethod" {
                return "byte";
            } else if name == "TimerPhase" {
                return "byte";
            }
        }
    }
    panic!("Unknown type {:#?}", ty);
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
    class LiveSplitCoreNative
    {"#)?;

    for function in functions {
        let name = function.name.to_string();
        let new_prefix = name.split('_').next().unwrap();
        if !prefix.is_empty() && new_prefix != prefix {
            writeln!(writer, "")?;
        }
        prefix = new_prefix.to_string();

        write!(writer,
               r#"
        [DllImport("livesplit-core")]
        public static extern {} {}("#,
               function.output
                   .map(get_type)
                   .map(|t| if t == "string" { "LSCoreString" } else { t })
                   .unwrap_or("void"),
               function.name)?;

        for (i, &(name, typ)) in function.inputs.iter().enumerate() {
            if i != 0 {
                write!(writer, ", ")?;
            }
            write!(writer,
                   "{} {}",
                   get_type(typ),
                   name.map(|n| if n == "this" {
                           Symbol::intern("self")
                       } else if n == "this_drop" {
                           Symbol::intern("self_drop")
                       } else {
                           n
                       })
                       .unwrap_or_else(|| Symbol::intern("parameter")))?;
        }

        write!(writer, ");")?;
    }

    writeln!(writer,
             "{}",
             r#"
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
