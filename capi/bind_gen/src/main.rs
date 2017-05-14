extern crate syntex_syntax;
extern crate heck;

mod c;
mod csharp;
mod emscripten;
mod java_jna;
mod java_jni;
mod java_jni_cpp;
mod node;
mod python;
mod ruby;

use std::path::Path;
use syntex_syntax::abi::Abi;
use syntex_syntax::ast::{ItemKind, Visibility, PatKind, TyKind, Mutability, FunctionRetTy};
use syntex_syntax::parse::{ParseSess, parse_crate_from_file};

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
            if name.starts_with("Owned") {
                name = name["Owned".len()..].to_string();
            }
            if name == "TimingMethod" {
                name = String::from("u8");
            } else if name == "TimerPhase" {
                name = String::from("u8");
            }
            let is_custom = match &name as &str {
                "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" | "()" | "bool" |
                "c_char" | "usize" | "isize" | "f32" | "f64" | "Json" => false,
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
        if let &ItemKind::Mod(ref module) = &item.node {
            for item in &module.items {
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
                                let name = if let &PatKind::Ident(_, ref ident, _) =
                                    &i.pat.node {
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

    write_files(&fns_to_classes(functions)).unwrap();
}

use std::collections::BTreeMap;

fn fns_to_classes(functions: Vec<Function>) -> BTreeMap<String, Class> {
    let mut classes = BTreeMap::new();

    for function in functions {
        let class = classes
            .entry(function.class.clone())
            .or_insert_with(Class::default);
        let kind = if let Some(&(ref name, ref ty)) = function.inputs.get(0) {
            if name == "this" { Some(ty.kind) } else { None }
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

use std::fs::{File, create_dir_all};
use std::io::{BufWriter, Result};
use std::path::PathBuf;

fn write_files(classes: &BTreeMap<String, Class>) -> Result<()> {
    let mut path = PathBuf::from("..");
    path.push("bindings");

    create_dir_all(&path)?;

    path.push("livesplit_core_emscripten.js");
    emscripten::write(BufWriter::new(File::create(&path)?), classes, false)?;
    path.pop();

    path.push("livesplit_core_emscripten.ts");
    emscripten::write(BufWriter::new(File::create(&path)?), classes, true)?;
    path.pop();

    path.push("livesplit_core_node.js");
    node::write(BufWriter::new(File::create(&path)?), classes, false)?;
    path.pop();

    path.push("livesplit_core_node.ts");
    node::write(BufWriter::new(File::create(&path)?), classes, true)?;
    path.pop();

    path.push("LiveSplitCore.cs");
    csharp::write(BufWriter::new(File::create(&path)?), classes)?;
    path.pop();

    path.push("java-native-access");
    create_dir_all(&path)?;
    java_jna::write(&path, classes)?;
    path.pop();

    path.push("java-native-interface");
    create_dir_all(&path)?;
    java_jni::write(&path, classes)?;
    path.pop();

    path.push("LiveSplitCoreJNI.cpp");
    java_jni_cpp::write(BufWriter::new(File::create(&path)?), classes)?;
    path.pop();

    path.push("LiveSplitCore.rb");
    ruby::write(BufWriter::new(File::create(&path)?), classes)?;
    path.pop();

    path.push("livesplit_core.h");
    c::write(BufWriter::new(File::create(&path)?), classes)?;
    path.pop();

    path.push("livesplit_core.py");
    python::write(BufWriter::new(File::create(&path)?), classes)?;
    path.pop();

    Ok(())
}
