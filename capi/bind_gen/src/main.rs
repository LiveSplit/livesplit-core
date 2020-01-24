use structopt;

mod c;
mod csharp;
mod java;
mod jni_cpp;
mod kotlin;
mod node;
mod python;
mod ruby;
mod swift;
mod typescript;
mod wasm;
mod wasm_bindgen;

use std::fs::{create_dir_all, remove_dir_all, File};
use std::io::{BufWriter, Read, Result};
use std::path::PathBuf;
use std::rc::Rc;
use structopt::StructOpt;
use syn::{
    parse_file, FnArg, Item, ItemFn, Lit, Meta, Pat, ReturnType, Signature, Type as SynType,
    Visibility,
};

#[derive(StructOpt)]
#[structopt(about = "Generates bindings for livesplit-core")]
pub struct Opt {
    #[structopt(
        long = "ruby-lib-path",
        help = "The path of the library for the Ruby bindings",
        default_value = "../liblivesplit_core.so"
    )]
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
    comments: Vec<String>,
    class_comments: Rc<Vec<String>>,
}

impl Function {
    fn is_static(&self) -> bool {
        if let Some((name, _)) = self.inputs.get(0) {
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
    comments: Rc<Vec<String>>,
    static_fns: Vec<Function>,
    shared_fns: Vec<Function>,
    mut_fns: Vec<Function>,
    own_fns: Vec<Function>,
}

fn get_type(ty: &SynType) -> Type {
    match ty {
        SynType::Reference(reference) => {
            let mut ty = get_type(&reference.elem);
            ty.kind = if reference.mutability.is_some() {
                TypeKind::RefMut
            } else {
                TypeKind::Ref
            };
            ty
        }
        SynType::Ptr(ptr) => {
            let mut ty = get_type(&ptr.elem);
            ty.kind = if ptr.mutability.is_some() {
                TypeKind::RefMut
            } else {
                TypeKind::Ref
            };
            ty
        }
        SynType::Path(path) => {
            if let Some(segment) = path.path.segments.iter().last() {
                let mut name = segment.ident.to_string();
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
                Type {
                    kind: TypeKind::Value,
                    is_custom,
                    is_nullable,
                    name,
                }
            } else {
                panic!("Weird path")
            }
        }
        _ => panic!("Weird type"),
    }
}

fn main() {
    let opt = Opt::from_args();

    let mut contents = String::new();
    File::open("../src/lib.rs")
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();

    let file = parse_file(&contents).unwrap();

    let mut functions = Vec::new();

    for item in &file.items {
        if let Item::Mod(module) = item {
            contents.clear();
            File::open(format!("../src/{}.rs", module.ident))
                .unwrap()
                .read_to_string(&mut contents)
                .unwrap();
            let file = parse_file(&contents).unwrap();

            let class_comments = Rc::new(
                file.attrs
                    .iter()
                    .filter_map(|a| match a.parse_meta() {
                        Ok(Meta::NameValue(v)) => Some(v),
                        _ => None,
                    })
                    .filter(|m| m.path.is_ident("doc"))
                    .filter_map(|m| match m.lit {
                        Lit::Str(s) => Some(s.value().trim().to_string()),
                        _ => None,
                    })
                    .collect::<Vec<_>>(),
            );

            for item in &file.items {
                if let &Item::Fn(ItemFn {
                    vis: Visibility::Public(_),
                    attrs,
                    sig:
                        Signature {
                            abi,
                            ident,
                            inputs,
                            output,
                            ..
                        },
                    ..
                }) = &item
                {
                    if abi
                        .as_ref()
                        .and_then(|a| a.name.as_ref())
                        .map_or(false, |n| n.value() == "C")
                        && attrs
                            .iter()
                            .filter_map(|a| match a.parse_meta() {
                                Ok(Meta::Path(w)) => Some(w),
                                _ => None,
                            })
                            .any(|w| w.is_ident("no_mangle"))
                    {
                        let comments = attrs
                            .iter()
                            .filter_map(|a| match a.parse_meta() {
                                Ok(Meta::NameValue(v)) => Some(v),
                                _ => None,
                            })
                            .filter(|m| m.path.is_ident("doc"))
                            .filter_map(|m| match m.lit {
                                Lit::Str(s) => Some(s.value().trim().to_string()),
                                _ => None,
                            })
                            .collect();

                        let output = if let ReturnType::Type(_, ty) = output {
                            get_type(ty)
                        } else {
                            Type {
                                kind: TypeKind::Value,
                                is_custom: false,
                                is_nullable: false,
                                name: String::from("()"),
                            }
                        };

                        let inputs = inputs
                            .iter()
                            .map(|i| {
                                let c = match i {
                                    FnArg::Typed(c) => c,
                                    _ => panic!("Found a weird fn argument"),
                                };
                                let name = if let Pat::Ident(ident) = &*c.pat {
                                    ident.ident.to_string()
                                } else {
                                    String::from("parameter")
                                };
                                (name, get_type(&c.ty))
                            })
                            .collect();

                        let name = ident.to_string();
                        let class;
                        let method;
                        {
                            let mut splits = name.splitn(2, '_');
                            class = splits.next().unwrap().to_string();
                            method = splits.next().unwrap().to_string();
                        }

                        functions.push(Function {
                            name,
                            class,
                            method,
                            output,
                            inputs,
                            comments,
                            class_comments: class_comments.clone(),
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

        class.comments = function.class_comments.clone();

        let kind = if let Some((name, ty)) = function.inputs.get(0) {
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

fn write_files(classes: &BTreeMap<String, Class>, opt: &Opt) -> Result<()> {
    let mut path = PathBuf::from("..");
    path.push("bindings");

    remove_dir_all(&path).ok();
    create_dir_all(&path)?;

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

    path.push("wasm");
    create_dir_all(&path)?;
    {
        path.push("livesplit_core.js");
        wasm::write(BufWriter::new(File::create(&path)?), classes, false)?;
        path.pop();

        path.push("livesplit_core.ts");
        wasm::write(BufWriter::new(File::create(&path)?), classes, true)?;
        path.pop();
    }
    path.pop();

    path.push("wasm_bindgen");
    create_dir_all(&path)?;
    {
        path.push("index.js");
        wasm_bindgen::write(BufWriter::new(File::create(&path)?), classes, false)?;
        path.pop();

        path.push("index.ts");
        wasm_bindgen::write(BufWriter::new(File::create(&path)?), classes, true)?;
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
