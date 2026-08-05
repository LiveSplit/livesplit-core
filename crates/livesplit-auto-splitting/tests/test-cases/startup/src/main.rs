#[link(wasm_import_module = "env")]
unsafe extern "C" {
    safe fn runtime_set_tick_rate(ticks_per_second: f64);
    fn runtime_print_message(text_ptr: *const u8, text_len: usize);
    fn user_settings_add_bool(
        key_ptr: *const u8,
        key_len: usize,
        description_ptr: *const u8,
        description_len: usize,
        default_value: u32,
    ) -> u32;
}

#[unsafe(no_mangle)]
pub extern "C" fn update() {}

fn main() {
    let key = "enabled";
    let description = "Enabled";
    let enabled = unsafe {
        user_settings_add_bool(
            key.as_ptr(),
            key.len(),
            description.as_ptr(),
            description.len(),
            0,
        )
    };
    runtime_set_tick_rate(if enabled != 0 { 42.0 } else { 24.0 });

    let hang_key = "hang";
    let hang_description = "Hang during startup";
    let hang = unsafe {
        user_settings_add_bool(
            hang_key.as_ptr(),
            hang_key.len(),
            hang_description.as_ptr(),
            hang_description.len(),
            0,
        )
    };
    if hang != 0 {
        loop {
            std::hint::spin_loop();
        }
    }

    let message = "startup completed";
    unsafe { runtime_print_message(message.as_ptr(), message.len()) };
}
