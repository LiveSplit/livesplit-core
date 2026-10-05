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
use heck::ToLowerCamelCase;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File, create_dir_all, remove_dir_all},
    io::{BufWriter, Read, Result},
    path::PathBuf,
    process::Command,
    rc::Rc,
};
use syn::{
    Expr, ExprLit, FnArg, Item, ItemFn, Lit, Meta, MetaList, Pat, ReturnType, Signature, Token,
    Type as SynType, Visibility, parse_file, punctuated::Punctuated,
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

    /// Features to enable for the C API and the generated bindings.
    #[clap(long, value_delimiter = ',')]
    features: Vec<String>,

    /// Do not include the C API's default features.
    #[clap(long)]
    no_default_features: bool,

    /// Directory to write the generated bindings to.
    #[clap(long, default_value = "../bindings")]
    output_dir: PathBuf,
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

/// Returns the public method name to use in JavaScript-facing bindings.
///
/// A static `name` method collides with the built-in `Function.name` property
/// inherited by every JavaScript class constructor. Other target languages do
/// not have that constraint, so the raw C API and their generated bindings can
/// keep the concise name while only JavaScript exposes it as `displayName`.
fn javascript_method_name(method: &str, is_static: bool) -> String {
    let method = method.to_lower_camel_case();
    if is_static && method == "name" {
        "displayName".into()
    } else {
        method
    }
}

#[cfg(test)]
mod tests {
    use super::javascript_method_name;

    #[test]
    fn avoids_the_static_function_name_property_in_javascript() {
        assert_eq!(javascript_method_name("name", true), "displayName");
        assert_eq!(javascript_method_name("name", false), "name");
        assert_eq!(javascript_method_name("parse_locale", true), "parseLocale");
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

impl Class {
    fn has_function(&self, name: &str) -> bool {
        self.static_fns
            .iter()
            .chain(&self.shared_fns)
            .chain(&self.mut_fns)
            .chain(&self.own_fns)
            .any(|function| function.name == name)
    }
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
            ty.kind = if matches!(ptr.mutability, syn::PointerMutability::Mut(_)) {
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
    let features = resolve_features(&opt);

    let mut contents = fs::read_to_string("../src/lib.rs").unwrap();
    let file = parse_file(&contents).unwrap();

    let mut functions = Vec::new();

    for item in &file.items {
        let module = match item {
            Item::Mod(m) if attrs_enabled(&m.attrs, &features) => m,
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
                Item::Fn(i)
                    if matches!(i.vis, Visibility::Public(_))
                        && attrs_enabled(&i.attrs, &features) =>
                {
                    i
                }
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

/// Resolves the selected feature set using the C API's Cargo feature graph.
///
/// The binding generator deliberately reads Cargo's feature definitions rather
/// than maintaining a second dependency graph. Build scripts still pass the
/// same top-level feature list to Cargo and this executable, while aggregate
/// features such as `parsing` are expanded from the single authoritative
/// definition in the C API's manifest.
fn resolve_features(opt: &Opt) -> BTreeSet<String> {
    let output = Command::new("cargo")
        .args([
            "metadata",
            "--format-version=1",
            "--no-deps",
            "--manifest-path=../Cargo.toml",
        ])
        .output()
        .expect("failed to query the C API's Cargo features");
    assert!(
        output.status.success(),
        "failed to query the C API's Cargo features: {}",
        String::from_utf8_lossy(&output.stderr),
    );

    let metadata: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("Cargo returned invalid metadata");
    let package = metadata["packages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|package| package["name"] == "livesplit-core-capi")
        .expect("the C API package is missing from Cargo metadata");
    let definitions = package["features"]
        .as_object()
        .expect("the C API's feature definitions are missing");

    let mut pending = opt.features.clone();
    if !opt.no_default_features {
        pending.push("default".into());
    }

    let mut resolved = BTreeSet::new();
    while let Some(feature) = pending.pop() {
        if !resolved.insert(feature.clone()) {
            continue;
        }

        let dependencies = definitions
            .get(&feature)
            .unwrap_or_else(|| panic!("unknown C API feature `{feature}`"))
            .as_array()
            .unwrap();
        for dependency in dependencies {
            let dependency = dependency.as_str().unwrap();
            // Entries containing a slash enable a feature of a dependency.
            // `dep:` entries enable an optional dependency. Neither represents
            // a feature that can occur in this crate's cfg attributes.
            if !dependency.contains('/') && !dependency.starts_with("dep:") {
                pending.push(dependency.into());
            }
        }
    }

    resolved
}

/// Evaluates the feature-related portion of `cfg` attributes.
///
/// Target predicates are intentionally treated as enabled. The generated C,
/// C#, Java, and similar bindings describe the C ABI, while target-specific
/// wasm-bindgen exports are generated by wasm-bindgen itself. Treating unknown
/// predicates as enabled also preserves the historical union of native target
/// APIs, while feature predicates are still applied consistently.
fn attrs_enabled(attrs: &[syn::Attribute], features: &BTreeSet<String>) -> bool {
    attrs.iter().all(|attribute| {
        let Meta::List(list) = &attribute.meta else {
            return true;
        };
        if !list.path.is_ident("cfg") {
            return true;
        }
        eval_cfg(&list.parse_args().expect("invalid cfg attribute"), features)
    })
}

fn eval_cfg(meta: &Meta, features: &BTreeSet<String>) -> bool {
    match meta {
        Meta::NameValue(value) if value.path.is_ident("feature") => {
            let Expr::Lit(ExprLit {
                lit: Lit::Str(feature),
                ..
            }) = &value.value
            else {
                panic!("invalid feature cfg");
            };
            features.contains(&feature.value())
        }
        Meta::List(list) if list.path.is_ident("all") => list
            .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
            .unwrap()
            .iter()
            .all(|meta| eval_cfg(meta, features)),
        Meta::List(list) if list.path.is_ident("any") => list
            .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
            .unwrap()
            .iter()
            .any(|meta| eval_cfg(meta, features)),
        Meta::List(list) if list.path.is_ident("not") => {
            !eval_cfg(&list.parse_args().expect("invalid not cfg"), features)
        }
        _ => true,
    }
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
    let mut path = opt.output_dir.clone();

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
