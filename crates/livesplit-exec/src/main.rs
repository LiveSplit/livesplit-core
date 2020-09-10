use libc::prctl;
use std::env;
use std::os::unix::process::CommandExt;
use std::process;

fn main() {
    unsafe {
        prctl(libc::PR_SET_PTRACER, -1 /*PR_SET_PTRACER_ANY*/);
    }
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("usage: livesplit-exec <command>");
        process::exit(1);
    }

    env::set_var("LD_PRELOAD", {
        match env::var("LD_PRELOAD") {
            Ok(val) => "liblivesplit-exec.so:liblivesplit-exec32.so:".to_owned() + &val,
            Err(_) => "liblivesplit-exec.so:liblivesplit-exec32.so".to_owned(),
        }
    });

    process::Command::new(&args[1]).args(&args[2..]).exec();
}
