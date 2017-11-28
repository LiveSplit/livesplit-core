extern crate heck;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate syntex_syntax;

mod c;
mod csharp;
mod emscripten;
mod java;
mod jni_cpp;
mod kotlin;
mod node;
mod python;
mod ruby;
mod swift;
mod typescript;

use structopt::StructOpt;
use std::path::Path;
use syntex_syntax::abi::Abi;
use syntex_syntax::ast::{FunctionRetTy, ItemKind, Mutability, PatKind, TyKind, Visibility};
use syntex_syntax::parse::{parse_crate_from_file, ParseSess};
use syntex_syntax::codemap::FilePathMapping;

#[derive(StructOpt)]
#[structopt(about = "Generates bindings for livesplit-core")]
pub struct Opt {
    #[structopt(long = "ruby-lib-path", help = "The path of the library for the Ruby bindings",
                default_value = "../liblivesplit_core.so")]
    ruby_lib_path: String,
}

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
    is_nullable: bool,
    name: String,
}

#[derive(Debug)]
pub struct Function {
    name: String,
    class: String,
    method: String,
    inputs: Vec<(String, Type)>,
    output: Type,
}

impl Function {
    fn is_static(&self) -> bool {
        if let Some(&(ref name, _)) = self.inputs.get(0) {
            name != "this"
        } else {
            true
        }
    }

    fn has_return_type(&self) -> bool {
        self.output.name != "()"
    }
}

#[derive(Debug, Default)]
pub struct Class {
    static_fns: Vec<Function>,
    shared_fns: Vec<Function>,
    mut_fns: Vec<Function>,
    own_fns: Vec<Function>,
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
            let is_nullable = if name.starts_with("Nullable") {
                name = name["Nullable".len()..].to_string();
                true
            } else {
                false
            };

            if name.starts_with("Owned") {
                name = name["Owned".len()..].to_string();
            }
            if name == "TimingMethod" {
                name = String::from("u8");
            } else if name == "TimerPhase" {
                name = String::from("u8");
            }
            let is_custom = match &name as &str {
                "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" | "()" | "bool"
                | "c_char" | "usize" | "isize" | "f32" | "f64" | "Json" => false,
                _ => true,
            };
            return Type {
                kind: TypeKind::Value,
                is_custom,
                is_nullable,
                name,
            };
        }
    }
    panic!("Unknown type {:#?}", ty);
}

fn main() {
    let opt = Opt::from_args();

    let sess = ParseSess::new(FilePathMapping::empty());
    let ast = parse_crate_from_file(Path::new("../src/lib.rs"), &sess).unwrap();
    let items = ast.module.items;

    let mut functions = Vec::new();

    for item in &items {
        if let &ItemKind::Mod(ref module) = &item.node {
            for item in &module.items {
                if let &ItemKind::Fn(ref decl, _, _, Abi::C, _, _) = &item.node {
                    if item.vis == Visibility::Public
                        && item.attrs.iter().any(|a| a.check_name("no_mangle"))
                    {
                        let output = if let &FunctionRetTy::Ty(ref output) = &decl.output {
                            get_type(&output.node)
                        } else {
                            Type {
                                kind: TypeKind::Value,
                                is_custom: false,
                                is_nullable: false,
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

                        let name = item.ident.name.to_string();
                        let class;
                        let method;
                        {
                            let mut splits = name.splitn(2, '_');
                            class = splits.next().unwrap().to_string();
                            method = splits.next().unwrap().to_string();
                        }

                        functions.push(Function {
                            name: name,
                            class: class,
                            method: method,
                            output: output,
                            inputs: inputs,
                        });
                    }
                }
            }
        }
    }

    write_files(&fns_to_classes(functions), &opt).unwrap();
}

use std::collections::BTreeMap;

fn fns_to_classes(functions: Vec<Function>) -> BTreeMap<String, Class> {
    let mut classes = BTreeMap::new();

    for function in functions {
        let class = classes
            .entry(function.class.clone())
            .or_insert_with(Class::default);
        let kind = if let Some(&(ref name, ref ty)) = function.inputs.get(0) {
            if name == "this" {
                Some(ty.kind)
            } else {
                None
            }
        } else {
            None
        };
        match kind {
            Some(TypeKind::Value) => class.own_fns.push(function),
            Some(TypeKind::Ref) => class.shared_fns.push(function),
            Some(TypeKind::RefMut) => class.mut_fns.push(function),
            None => class.static_fns.push(function),
        }
    }

    classes
}

use std::fs::{create_dir_all, remove_dir_all, File};
use std::io::{BufWriter, Result};
use std::path::PathBuf;

fn write_files(classes: &BTreeMap<String, Class>, opt: &Opt) -> Result<()> {
    let mut path = PathBuf::from("..");
    path.push("bindings");

    remove_dir_all(&path).ok();
    create_dir_all(&path)?;

    path.push("emscripten");
    create_dir_all(&path)?;
    {
        path.push("livesplit_core.js");
        emscripten::write(BufWriter::new(File::create(&path)?), classes, false)?;
        path.pop();

        path.push("livesplit_core.ts");
        emscripten::write(BufWriter::new(File::create(&path)?), classes, true)?;
        path.pop();
    }
    path.pop();

    path.push("node");
    create_dir_all(&path)?;
    {
        path.push("livesplit_core.js");
        node::write(BufWriter::new(File::create(&path)?), classes, false)?;
        path.pop();

        path.push("livesplit_core.ts");
        node::write(BufWriter::new(File::create(&path)?), classes, true)?;
        path.pop();
    }
    path.pop();

    path.push("LiveSplitCore.cs");
    csharp::write(BufWriter::new(File::create(&path)?), classes)?;
    path.pop();

    path.push("java");
    create_dir_all(&path)?;
    java::write(&path, classes)?;
    path.pop();

    path.push("kotlin");
    create_dir_all(&path)?;
    kotlin::write(&path, classes)?;
    path.pop();

    path.push("LiveSplitCore.rb");
    ruby::write(BufWriter::new(File::create(&path)?), classes, opt)?;
    path.pop();

    path.push("livesplit_core.h");
    c::write(BufWriter::new(File::create(&path)?), classes)?;
    path.pop();

    path.push("livesplit_core.py");
    python::write(BufWriter::new(File::create(&path)?), classes)?;
    path.pop();

    path.push("swift");
    create_dir_all(&path)?;
    swift::write(&path, classes)?;
    path.pop();

    Ok(())
}
