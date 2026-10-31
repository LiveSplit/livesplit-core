use livesplit_auto_splitting::{Config, CreationError, Runtime};
use std::{error::Error, io};

#[test]
fn creation_errors_preserve_wasmtime_context_and_root_cause() {
    type WrapError = fn(wasmtime::Error) -> CreationError;
    let cases: [(WrapError, &str); 6] = [
        (
            |source| CreationError::EngineCreation { source },
            "Failed creating the WebAssembly engine.",
        ),
        (
            |source| CreationError::ModuleLoading { source },
            "Failed loading the WebAssembly module.",
        ),
        (
            |source| CreationError::LinkFunction {
                name: "timer_start",
                source,
            },
            "Failed linking the function `timer_start` to the WebAssembly module.",
        ),
        (
            |source| CreationError::ModuleInstantiation { source },
            "Failed instantiating the WebAssembly module.",
        ),
        (
            |source| CreationError::MissingUpdateFunction { source },
            "The WebAssembly module has no exported function called `update`, which is a required function.",
        ),
        (
            |source| CreationError::Wasi { source },
            "Failed linking the WebAssembly System Interface (WASI).",
        ),
    ];

    for (wrap, message) in cases {
        let error = wrap(
            wasmtime::Error::new(io::Error::new(io::ErrorKind::InvalidData, "root cause"))
                .context("Wasmtime context"),
        );
        assert_eq!(error.to_string(), message);

        let context = error.source().unwrap();
        assert_eq!(context.to_string(), "Wasmtime context");
        let source = context.source().unwrap();
        let source = source.downcast_ref::<io::Error>().unwrap();
        assert_eq!(source.kind(), io::ErrorKind::InvalidData);
        assert_eq!(source.to_string(), "root cause");
        assert!(source.source().is_none());
    }

    let error = CreationError::MissingMemory;
    assert_eq!(
        error.to_string(),
        "The WebAssembly module has no exported memory called `memory`, which is a requirement."
    );
    assert!(error.source().is_none());
}

#[test]
fn invalid_module_keeps_its_wasmtime_error() {
    let runtime = Runtime::new(Config::default()).unwrap();
    let error = match runtime.compile(b"invalid module") {
        Ok(_) => panic!("invalid WebAssembly must fail to compile"),
        Err(error) => error,
    };
    assert!(matches!(error, CreationError::ModuleLoading { .. }));
    assert_eq!(error.to_string(), "Failed loading the WebAssembly module.");
    assert!(error.source().is_some());
}
