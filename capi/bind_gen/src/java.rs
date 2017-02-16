use std::io::{Write, Result};
use {Function, Type, TypeKind};

fn to_camel_case(snake_case: &str) -> String {
    let mut camel_case = String::new();

    for (u, split) in snake_case.split('_').enumerate() {
        for (i, c) in split.char_indices() {
            if u != 0 && i == 0 {
                camel_case.extend(c.to_uppercase());
            } else {
                camel_case.push(c);
            }
        }
    }

    camel_case
}

fn get_type(ty: &Type) -> &str {
    match (ty.kind, ty.name.as_str()) {
        (TypeKind::Ref, "c_char") => "String",
        (TypeKind::Ref, _) |
        (TypeKind::RefMut, _) => "Pointer",
        (_, t) if !ty.is_custom => {
            match t {
                "i8" => "byte",
                "i16" => "short",
                "i32" => "int",
                "i64" => "long",
                "u8" => "byte",
                "u16" => "short",
                "u32" => "int",
                "u64" => "long",
                "usize" => "NativeLong", // Not really correct
                "f32" => "float",
                "f64" => "double",
                "bool" => "byte",
                "()" => "void",
                "c_char" => "byte",
                x => x,
            }
        }
        _ => "Pointer",
    }
}

pub fn write<W: Write>(mut writer: W, functions: &[Function]) -> Result<()> {
    let mut prefix = String::from("");

    write!(writer,
           "{}",
           r#"package livesplitcore;

import com.sun.jna.*;
import java.lang.reflect.Method;
import java.util.HashMap;

public interface LiveSplitCore extends Library {
    static class Mapper implements FunctionMapper {
        @Override
        public String getFunctionName(NativeLibrary nativeLibrary, Method method) {
            String name = method.getName();
            StringBuilder builder = new StringBuilder();
            boolean foundUnderscore = false;
            for (int i = 0; i < name.length(); i++) {
                char c = name.charAt(i);
                if (foundUnderscore) {
                    if (Character.isUpperCase(c)) {
                        builder.append('_');
                        c = Character.toLowerCase(c);
                    }
                } else if (c == '_') {
                    foundUnderscore = true;
                }
                builder.append(c);
            }
            return builder.toString();
        }
    }

    LiveSplitCore INSTANCE = (LiveSplitCore) Native.loadLibrary("livesplit-core", LiveSplitCore.class, new HashMap() {
        {
            put(Library.OPTION_FUNCTION_MAPPER, new Mapper());
        }
    });
"#)?;

    for function in functions {
        let name = function.name.to_string();
        let mut splits = name.splitn(2, '_');
        let new_prefix = splits.next().unwrap();
        let postfix = splits.next().unwrap();
        if !prefix.is_empty() && new_prefix != prefix {
            writeln!(writer, "")?;
        }
        prefix = new_prefix.to_string();

        write!(writer,
               r#"
    {} {}("#,
               get_type(&function.output),
               format!("{}_{}", prefix, to_camel_case(postfix)))?;

        for (i, &(ref name, ref typ)) in function.inputs.iter().enumerate() {
            if i != 0 {
                write!(writer, ", ")?;
            }
            write!(writer,
                   "{} {}",
                   get_type(typ),
                   if name == "this" {
                       String::from("self")
                   } else {
                       to_camel_case(name)
                   })?;
        }

        write!(writer, ");")?;
    }

    writeln!(writer,
             "{}",
             r#"
}"#)
}
