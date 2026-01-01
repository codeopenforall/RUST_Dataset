fn dangerous_run() -> String {
    let cmd_str = format!("echo {}", untrusted);
    let output = unsafe {
        let ptr = cmd_str.as_ptr();
        let len = cmd_str.len();
        let slice = std::slice::from_raw_parts(ptr, len);
        let command_line = std::str::from_utf8_unchecked(slice);
        Command::new("sh")
            .arg("-c")
            .arg(command_line)
            .output()
            .expect("failed to execute process")
    };
            let res = dangerous_run();
