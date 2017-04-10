#[macro_use]
extern crate dyon;
extern crate livesplit_deep_pointer as deep_pointer;

use dyon::{Runtime, Module, RustObject, Dfn, Lt, Type, load_str, Variable};
use deep_pointer::{Process, DeepPointer};
use std::sync::{Arc, Mutex};

pub struct Script {
    runtime: Runtime,
    module: Arc<Module>,
}

enum DynamicPtrType {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
}

struct DynamicPtr {
    ty: DynamicPtrType,
    ptr: DeepPointer,
}

impl DynamicPtr {
    fn deref(&self, process: &Process) -> deep_pointer::Result<Variable> {
        use self::DynamicPtrType::*;

        Ok(match self.ty {
            U8 => Variable::f64(self.ptr.deref::<u8>(process)? as _),
            U16 => Variable::f64(self.ptr.deref::<u16>(process)? as _),
            U32 => Variable::f64(self.ptr.deref::<u32>(process)? as _),
            U64 => Variable::f64(self.ptr.deref::<u64>(process)? as _),
            I8 => Variable::f64(self.ptr.deref::<i8>(process)? as _),
            I16 => Variable::f64(self.ptr.deref::<i16>(process)? as _),
            I32 => Variable::f64(self.ptr.deref::<i32>(process)? as _),
            I64 => Variable::f64(self.ptr.deref::<i64>(process)? as _),
        })
    }
}

impl Script {
    pub fn new<S>(script: S) -> Result<Self, String>
        where S: Into<String>
    {
        let runtime = Runtime::new();
        let mut module = Module::new();

        module.add(Arc::new("x".into()),
                   hex,
                   Dfn {
                       lts: vec![Lt::Default],
                       tys: vec![Type::Text],
                       ret: Type::Result(Box::new(Type::F64)),
                   });

        let ty_custom_object = Type::AdHoc(Arc::new("Process".into()), Box::new(Type::Any));
        module.add(Arc::new("process".into()),
                   process,
                   Dfn {
                       lts: vec![Lt::Default],
                       tys: vec![Type::Text],
                       ret: Type::Result(Box::new(ty_custom_object)),
                   });

        let ty_custom_object = Type::AdHoc(Arc::new("DeepPointer".into()), Box::new(Type::Any));
        module.add(Arc::new("pointer".into()),
                   pointer,
                   Dfn {
                       lts: vec![Lt::Default, Lt::Default],
                       tys: vec![Type::Text, Type::Array(Box::new(Type::F64))],
                       ret: Type::Result(Box::new(ty_custom_object)),
                   });

        load_str("Script", Arc::new(script.into()), &mut module)?;

        Ok(Self {
            runtime: runtime,
            module: Arc::new(module),
        })
    }

    pub fn exec(&mut self) -> Result<(), String> {
        self.runtime.run(&self.module)
    }
}

fn rust_obj<T>(obj: T) -> Arc<Mutex<T>> {
    Arc::new(Mutex::new(obj))
}

dyon_fn! {
fn hex(lit: String) -> Result<f64, String> {
    i64::from_str_radix(&lit, 16)
        .map(|n| n as _)
        .map_err(|_| format!("Couldn't parse '{}' as hexadecimal number", lit))
}
}

fn int_lit(var: Variable) -> Result<f64, String> {
    match var {
        Variable::F64(x, _) => Ok(x),
        Variable::Text(lit) => {
            i64::from_str_radix(&lit, 16)
                .map(|n| n as _)
                .map_err(|_| format!("Couldn't parse '{}' as hexadecimal number", lit))
        }
        _ => Err(String::from("Parameter is not a Number")),
    }
}

dyon_fn! {
fn process(name: String) -> Result<RustObject, String> {
    if let Ok(process) = Process::with_name(&name) {
        Ok(rust_obj(process))
    } else {
        Err(String::from("Process not found"))
    }
}
}

dyon_fn! {
fn pointer(module: String, offsets: Vec<Variable>) -> Result<RustObject, String> {
    let offsets = offsets.into_iter()
        .map(|o| int_lit(o).map(|o| o as i64))
        .collect::<Result<Vec<i64>, String>>()?;
    Ok(rust_obj(DeepPointer::new(module, offsets)))
}
}

#[test]
fn test() {
    let mut script = match Script::new(r#"
fn main() {
    notepad := unwrap(process("notepad.exe"))
    ptr := unwrap(pointer("tiptsf.dll", ["7A000", "4c0", "2b8"]))
}
    "#) {
        Ok(script) => script,
        Err(e) => {
            ::dyon::error(Err(e));
            panic!("Syntax Error")
        }
    };

    assert!(!::dyon::error(script.exec()));
}
