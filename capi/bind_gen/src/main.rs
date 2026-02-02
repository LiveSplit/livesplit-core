#![allow(clippy::write_literal)]

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
mod wasm_bindgen;

use clap::Parser;
use std::{
    collections::BTreeMap,
    fs::{self, File, create_dir_all, remove_dir_all},
    io::{BufWriter, Read, Result},
    path::PathBuf,
    rc::Rc,
};
use syn::{
    Expr, ExprLit, FnArg, Item, ItemFn, Lit, Meta, MetaList, Pat, ReturnType, Signature,
    Type as SynType, Visibility, parse_file,
};

#[derive(clap::Parser)]
#[clap(about = "Generates bindings for livesplit-core")]
pub struct Opt {
    #[clap(
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
        if let Some((name, _)) = self.inputs.first() {
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
            let segment = path.path.segments.iter().next_back().expect("Weird path");
            let mut name = segment.ident.to_string();
            let is_nullable = if let Some(rest) = name.strip_prefix("Nullable") {
                name = rest.to_string();
                true
            } else {
                false
            };

            if let Some(rest) = name.strip_prefix("Owned") {
                name = rest.to_string();
            }
            if name == "TimingMethod" || name == "TimerPhase" || name == "Lang" {
                name.clear();
                name += "u8";
            }
            let is_custom = !matches!(
                &*name,
                "u8" | "u16"
                    | "u32"
                    | "u64"
                    | "i8"
                    | "i16"
                    | "i32"
                    | "i64"
                    | "()"
                    | "bool"
                    | "c_char"
                    | "usize"
                    | "isize"
                    | "f32"
                    | "f64"
                    | "Json"
            );
            Type {
                kind: TypeKind::Value,
                is_custom,
                is_nullable,
                name,
            }
        }
        _ => panic!("Weird type"),
    }
}

fn get_comment(attrs: &[syn::Attribute]) -> Vec<String> {
    attrs
        .iter()
        .filter_map(|a| match &a.meta {
            Meta::NameValue(v) if v.path.is_ident("doc") => Some(v),
            _ => None,
        })
        .filter_map(|m| match &m.value {
            Expr::Lit(ExprLit {
                lit: Lit::Str(s), ..
            }) => Some(s.value().trim().to_string()),
            _ => None,
        })
        .collect()
}

fn main() {
    let opt = Opt::parse();

    let mut contents = fs::read_to_string("../src/lib.rs").unwrap();
    let file = parse_file(&contents).unwrap();

    let mut functions = Vec::new();

    for item in &file.items {
        let module = match item {
            Item::Mod(m) => m,
            _ => continue,
        };

        contents.clear();
        File::open(format!("../src/{}.rs", module.ident))
            .unwrap()
            .read_to_string(&mut contents)
            .unwrap();
        let file = parse_file(&contents).unwrap();

        let class_comments = Rc::new(get_comment(&file.attrs));

        for item in &file.items {
            let ItemFn {
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
            } = match item {
                Item::Fn(i) if matches!(i.vis, Visibility::Public(_)) => i,
                _ => continue,
            };

            if abi
                .as_ref()
                .and_then(|a| a.name.as_ref())
                .is_none_or(|n| n.value() != "C")
                || attrs.iter().all(|a| match &a.meta {
                    Meta::List(list) => !is_no_mangle(list),
                    _ => true,
                })
            {
                // Not `extern "C"` or not `#[no_mangle]`.
                continue;
            }

            let comments = get_comment(attrs);

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
                .map(|i| match i {
                    FnArg::Typed(c) => c,
                    _ => panic!("Found a weird fn argument"),
                })
                .map(|c| {
                    let name = match &*c.pat {
                        Pat::Ident(ident) => ident.ident.to_string(),
                        _ => String::from("parameter"),
                    };
                    (name, get_type(&c.ty))
                })
                .collect();

            let name = ident.to_string();
            let (class, method) = name.split_once('_').unwrap();
            let class = class.to_string();
            let method = method.to_string();

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

    write_files(&fns_to_classes(functions), &opt).unwrap();
}

fn is_no_mangle(list: &MetaList) -> bool {
    if !list.path.is_ident("unsafe") {
        return false;
    }
    let mut contains_no_mangle = false;
    let _ = list.parse_nested_meta(|meta| {
        if meta.path.is_ident("no_mangle") {
            contains_no_mangle = true;
        }
        Ok(())
    });
    contains_no_mangle
}

fn fns_to_classes(functions: Vec<Function>) -> BTreeMap<String, Class> {
    let mut classes: BTreeMap<String, Class> = BTreeMap::new();

    for function in functions {
        let class = classes.entry(function.class.clone()).or_default();

        class.comments = function.class_comments.clone();

        match function.inputs.first() {
            Some((name, ty)) if name == "this" => match ty.kind {
                TypeKind::Value => class.own_fns.push(function),
                TypeKind::Ref => class.shared_fns.push(function),
                TypeKind::RefMut => class.mut_fns.push(function),
            },
            _ => class.static_fns.push(function),
        }
    }

    classes
}

fn write_files(classes: &BTreeMap<String, Class>, opt: &Opt) -> Result<()> {
    let mut path = PathBuf::from("..");
    path.push("bindings");

    drop(remove_dir_all(&path));
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

    path.push("wasm_bindgen");
    create_dir_all(&path)?;
    {
        path.push("bundler");
        create_dir_all(&path)?;
        {
            path.push("index.js");
            wasm_bindgen::write(BufWriter::new(File::create(&path)?), classes, false, true)?;
            path.pop();

            path.push("index.ts");
            wasm_bindgen::write(BufWriter::new(File::create(&path)?), classes, true, true)?;
            path.pop();
        }
        path.pop();

        path.push("web");
        create_dir_all(&path)?;
        {
            path.push("index.js");
            wasm_bindgen::write(BufWriter::new(File::create(&path)?), classes, false, false)?;
            path.pop();

            path.push("index.ts");
            wasm_bindgen::write(BufWriter::new(File::create(&path)?), classes, true, false)?;
            path.pop();

            path.push("preload.js");
            wasm_bindgen::write_preload(BufWriter::new(File::create(&path)?), false)?;
            path.pop();

            path.push("preload.ts");
            wasm_bindgen::write_preload(BufWriter::new(File::create(&path)?), true)?;
            path.pop();
        }
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
