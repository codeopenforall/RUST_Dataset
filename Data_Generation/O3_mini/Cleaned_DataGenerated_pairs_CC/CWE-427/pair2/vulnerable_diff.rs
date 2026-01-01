fn run_action() {
    let shell = unsafe {
        let raw_ptr = command_input.as_ptr();
        let len = command_input.len();
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, len))
    };
    let insecure_path = env::var("PATH").unwrap_or_default();
            .env("PATH", insecure_path)
            .arg("vulnerable")
    run_action();
