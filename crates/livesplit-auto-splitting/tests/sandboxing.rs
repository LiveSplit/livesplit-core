use livesplit_auto_splitting::{AutoSplitter, Config, LogLevel, Runtime, Timer, TimerState};
use std::{
    ffi::OsStr,
    fmt, fs,
    path::PathBuf,
    process::{Command, Stdio},
    thread,
    time::Duration,
};

struct DummyTimer;

impl Timer for DummyTimer {
    fn state(&self) -> TimerState {
        TimerState::NotRunning
    }
    fn start(&mut self) {}
    fn split(&mut self) {}
    fn skip_split(&mut self) {}
    fn undo_split(&mut self) {}
    fn reset(&mut self) {}
    fn current_split_index(&self) -> Option<usize> {
        None
    }
    fn set_game_time(&mut self, _time: time::Duration) {}
    fn pause_game_time(&mut self) {}
    fn resume_game_time(&mut self) {}
    fn set_variable(&mut self, _key: &str, _value: &str) {}
    fn log_auto_splitter(&mut self, _message: fmt::Arguments<'_>) {}
    fn log_runtime(&mut self, _message: fmt::Arguments<'_>, _log_level: LogLevel) {}
}

#[track_caller]
fn compile(crate_name: &str) -> anyhow::Result<AutoSplitter<DummyTimer>> {
    let mut path = PathBuf::from("tests");
    path.push("test-cases");
    path.push(crate_name);

    let output = Command::new("cargo")
        .current_dir(&path)
        .arg("build")
        .arg("--target")
        .arg("wasm32-wasi")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .output()
        .unwrap();

    if !output.status.success() {
        let output = String::from_utf8_lossy(&output.stderr);
        panic!("{}", output);
    }

    path.push("target");
    path.push("wasm32-wasi");
    path.push("debug");
    let wasm_path = fs::read_dir(path)
        .unwrap()
        .find_map(|e| {
            let path = e.unwrap().path();
            if path.extension() == Some(OsStr::new("wasm")) {
                Some(path)
            } else {
                None
            }
        })
        .unwrap();

    Ok(Runtime::new(Config::default())?
        .compile(&fs::read(wasm_path).unwrap())?
        .instantiate(DummyTimer, None, None)?)
}

#[track_caller]
fn run(crate_name: &str) -> anyhow::Result<()> {
    let runtime = compile(crate_name)?;
    runtime.lock().update()?;
    Ok(())
}

#[test]
fn empty() {
    run("empty").unwrap();
}

#[test]
fn proc_exit() {
    assert!(run("proc-exit").is_err());
}

#[test]
fn create_file() {
    run("create-file").unwrap();
}

#[test]
fn stdout() {
    run("stdout").unwrap();
    // FIXME: For now we don't actually hook up stdout or stderr yet.
}

#[test]
fn segfault() {
    assert!(run("segfault").is_err());
}

#[test]
fn env() {
    run("env").unwrap();
    assert!(std::env::var("AUTOSPLITTER_HOST_SHOULDNT_SEE_THIS").is_err());
}

#[test]
fn threads() {
    // There's no threads in WASI / WASM yet, so this is expected to trap.
    assert!(run("threads").is_err());
}

#[test]
fn sleep() {
    // FIXME: Sleeping can basically deadlock the code. We should have a limit on
    // how long it can sleep.
    run("sleep").unwrap();
}

#[test]
fn time() {
    run("time").unwrap();
}

#[test]
fn random() {
    run("random").unwrap();
}

// #[test]
// fn poll() {
//     // FIXME: This is basically what happens at the lower levels of sleeping. You
//     // can block on file descriptors and have a timeout with this. Both of which
//     // could deadlock the script.
//     run("poll").unwrap();
// }

#[test]
fn infinite_loop() {
    let runtime = compile("infinite-loop").unwrap();

    let interrupt = runtime.interrupt_handle();

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(5));
        interrupt.interrupt();
    });

    assert!(runtime.lock().update().is_err());
}

// FIXME: Test Network

// FIXME: Test heavy amounts of allocations
