use libc::{c_int, c_ulong, dlsym};

#[no_mangle]
pub unsafe extern "C" fn fork() -> c_int {
    let real_fork: extern "C" fn() -> c_int =
        std::mem::transmute(dlsym(libc::RTLD_NEXT, "fork\0".as_ptr().cast()));
    // Get the real prctl as we don't want to call our hooked version.
    let real_prctl: extern "C" fn(option: c_int, ...) -> c_int =
        std::mem::transmute(dlsym(libc::RTLD_NEXT, "prctl\0".as_ptr().cast()));

    let pid = real_fork();

    if pid == 0 {
        real_prctl(libc::PR_SET_PTRACER, -1 /*PR_SET_PTRACER_ANY*/);
    }

    pid
}

#[no_mangle]
pub unsafe extern "C" fn prctl(
    option: c_int,
    arg1: c_ulong,
    arg2: c_ulong,
    arg3: c_ulong,
    arg4: c_ulong,
    arg5: c_ulong,
) -> c_int {
    let real_prctl: extern "C" fn(option: c_int, ...) -> c_int =
        std::mem::transmute(dlsym(libc::RTLD_NEXT, "prctl\0".as_ptr().cast()));

    if option == libc::PR_SET_PTRACER {
        real_prctl(libc::PR_SET_PTRACER, -1 /*PR_SET_PTRACER_ANY*/)
    } else {
        real_prctl(option, arg1, arg2, arg3, arg4, arg5)
    }
}
