extern crate syntex_syntax;

mod c;
mod csharp;
mod javascript;
mod java;
mod ruby;

use std::path::Path;
use syntex_syntax::abi::Abi;
use syntex_syntax::ast::{ItemKind, Visibility, PatKind, TyKind, Mutability, FunctionRetTy};
use syntex_syntax::parse::{ParseSess, parse_crate_from_file};
use syntex_syntax::symbol::Symbol;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum TypeKind {
    Value,
    Ref,
    RefMut,
}

#[derive(Debug)]
pub struct Type {
    kind: TypeKind,
    is_custom: bool,
    name: String,
}

#[derive(Debug)]
pub struct Function {
    name: Symbol,
    inputs: Vec<(String, Type)>,
    output: Type,
}

fn get_type(ty: &TyKind) -> Type {
    if let &TyKind::Ptr(ref ptr) = ty {
        let mut ty = get_type(&ptr.ty.node);
        ty.kind = if ptr.mutbl == Mutability::Mutable {
            TypeKind::RefMut
        } else {
            TypeKind::Ref
        };
        return ty;
    } else if let &TyKind::Path(_, ref path) = ty {
        if let Some(segment) = path.segments.last() {
            let mut name = segment.identifier.name.to_string();
            if name == "TimingMethod" {
                name = String::from("u8");
            } else if name == "TimerPhase" {
                name = String::from("u8");
            }
            let is_custom = match &name as &str {
                "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" | "()" | "bool" |
                "c_char" | "usize" | "isize" | "f32" | "f64" => false,
                _ => true,
            };
            return Type {
                kind: TypeKind::Value,
                is_custom: is_custom,
                name: name,
            };
        }
    }
    panic!("Unknown type {:#?}", ty);
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
                    get_type(&output.node)
                } else {
                    Type {
                        kind: TypeKind::Value,
                        is_custom: false,
                        name: String::from("()"),
                    }
                };

                let inputs = decl.inputs
                    .iter()
                    .map(|i| {
                        let name = if let &PatKind::Ident(_, ref ident, _) = &i.pat.node {
                            ident.node.name.to_string()
                        } else {
                            String::from("parameter")
                        };
                        (name, get_type(&i.ty.node))
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
        path.push("LiveSplitCore.java");
        java::write(BufWriter::new(File::create(&path)?), functions)?;
    }

    {
        let mut path = path.clone();
        path.push("LiveSplitCore.rb");
        ruby::write(BufWriter::new(File::create(&path)?), functions)?;
    }

    {
        let mut path = path.clone();
        path.push("livesplit_core.h");
        c::write(BufWriter::new(File::create(&path)?), functions)?;
    }

    Ok(())
}
