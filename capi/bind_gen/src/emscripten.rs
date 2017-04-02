use std::io::{Write, Result};
use {Function, Type, TypeKind};
use heck::MixedCase;

fn get_type(ty: &Type) -> &str {
    match (ty.kind, ty.name.as_str()) {
        (TypeKind::Ref, "c_char") => r#""string""#,
        (TypeKind::Value, "()") => "null",
        _ => r#""number""#,
    }
}

pub fn write<W: Write>(mut writer: W, functions: &[Function]) -> Result<()> {
    let mut prefix = String::from("");

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
               "var {2}_{3} = ls.cwrap('{0}', {1}, [",
               &function.name,
               get_type(&function.output),
               prefix,
               postfix.to_mixed_case())?;

        for (i, &(_, ref typ)) in function.inputs.iter().enumerate() {
            if i != 0 {
                write!(writer, ", ")?;
            }
            write!(writer, "{}", get_type(typ))?;
        }

        writeln!(writer, "]);")?;
    }

    Ok(())
}
