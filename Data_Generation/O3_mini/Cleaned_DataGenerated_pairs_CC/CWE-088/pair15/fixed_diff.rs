        let output = Command::new("echo")
            .arg(&self.input)
            .output()
            .map_err(|e| format!("Failed to execute: {}", e))?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
