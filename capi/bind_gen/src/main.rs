extern crate syntex_syntax;

mod c;
mod csharp;
mod javascript;

use std::path::Path;
use syntex_syntax::abi::Abi;
use syntex_syntax::ast::{ItemKind, Visibility, PatKind, TyKind, FunctionRetTy};
use syntex_syntax::parse::{ParseSess, parse_crate_from_file};
use syntex_syntax::symbol::Symbol;

#[derive(Debug)]
pub struct Function<'a> {
    name: Symbol,
    inputs: Vec<(Option<Symbol>, &'a TyKind)>,
    output: Option<&'a TyKind>,
}

fn main() {
    let sess = ParseSess::new();
    let ast = parse_crate_from_file(Path::new("../src/lib.rs"), &sess).unwrap();
    let items = ast.module.items;

    let mut functions = Vec::new();

    for item in &items {
        if let &ItemKind::Fn(ref decl, _, _, Abi::C, _, _) = &item.node {
            if item.vis == Visibility::Public &&
               item.attrs.iter().any(|a| a.value.name == "no_mangle") {
                let output = if let &FunctionRetTy::Ty(ref output) = &decl.output {
                    Some(&output.node)
                } else {
                    None
                };

                let inputs = decl.inputs
                    .iter()
                    .map(|i| {
                        let name = if let &PatKind::Ident(_, ref ident, _) = &i.pat.node {
                            Some(ident.node.name)
                        } else {
                            None
                        };
                        (name, &i.ty.node)
                    })
                    .collect();

                functions.push(Function {
                    name: item.ident.name,
                    output: output,
                    inputs: inputs,
                });
            }
        }
    }

    write_files(&functions).unwrap();
}

use std::fs::{self, File};
use std::io::{BufWriter, Result};
use std::path::PathBuf;

fn write_files(functions: &[Function]) -> Result<()> {
    let mut path = PathBuf::from("..");
    path.push("bindings");

    fs::create_dir_all(&path)?;

    {
        let mut path = path.clone();
        path.push("livesplit_core.js");
        javascript::write(BufWriter::new(File::create(&path)?), functions)?;
    }

    {
        let mut path = path.clone();
        path.push("LiveSplitCoreNative.cs");
        csharp::write(BufWriter::new(File::create(&path)?), functions)?;
    }

    {
        let mut path = path.clone();
        path.push("livesplit_core.h");
        c::write(BufWriter::new(File::create(&path)?), functions)?;
    }

    Ok(())
}
