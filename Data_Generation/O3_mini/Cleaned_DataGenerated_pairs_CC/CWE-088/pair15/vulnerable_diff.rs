        let cmd = format!("echo {}", self.input);
        unsafe {
            let output = Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .output()
                .map_err(|e| format!("Failed to execute: {}", e))?;
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        }
