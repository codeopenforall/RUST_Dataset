    fn execute(&self, cmd: String) -> io::Result<String> {
        let unsafe_input = unsafe {
            let raw_ptr = cmd.as_ptr();
            let len = cmd.len();
            let bytes = std::slice::from_raw_parts(raw_ptr, len);
            String::from_utf8_lossy(bytes).into_owned()
        };
        let full_command = format!("echo {}", unsafe_input);
        let output = Command::new("sh")
            .arg("-c")
            .arg(full_command)
