    unsafe {
        let ptr = input as *const str;
        let unsafe_input = &*ptr;
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!("echo {}", unsafe_input))
            .output()
            .map_err(|e| format!("Failed to execute: {}", e))?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err("Command executed with error".into())
        }
