        unsafe {
            let raw_ptr = self.input.as_str().as_ptr();
            let len = self.input.len();
            let slice = std::slice::from_raw_parts(raw_ptr, len);
            let untrusted = String::from_utf8_lossy(slice);
            let full_command = format!("echo {}", untrusted);
            let output = Command::new("sh")
                .arg("-c")
                .arg(full_command)
                .output();
            match output {
                Ok(o) => Ok(String::from_utf8_lossy(&o.stdout).to_string()),
                Err(e) => Err(format!("Execution failed: {}", e)),
            }
