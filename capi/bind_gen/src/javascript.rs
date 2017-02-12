use std::io::{Write, Result};
use Function;
use syntex_syntax::ast::TyKind;

fn get_type(ty: &TyKind) -> &'static str {
    if let &TyKind::Ptr(ref ptr) = ty {
        if let &TyKind::Path(_, ref path) = &ptr.ty.node {
            if let Some(segment) = path.segments.last() {
                let name = segment.identifier.name;
                if name == "c_char" {
                    return "'string'";
                }
            }
        }
    }
    "'number'"
}

pub fn write<W: Write>(mut writer: W, functions: &[Function]) -> Result<()> {
    let mut prefix = String::from("");

    for function in functions {
        let name = function.name.to_string();
        let new_prefix = name.split('_').next().unwrap();
        if !prefix.is_empty() && new_prefix != prefix {
            writeln!(writer, "")?;
        }
        prefix = new_prefix.to_string();

        write!(writer,
               "var {0} = ls.cwrap('{0}', {1}, [",
               function.name,
               function.output.map_or("null", get_type))?;

        for (i, &(_, typ)) in function.inputs.iter().enumerate() {
            if i != 0 {
                write!(writer, ", ")?;
            }
            write!(writer, "{}", get_type(typ))?;
        }

        writeln!(writer, "]);")?;
    }

    Ok(())
}
