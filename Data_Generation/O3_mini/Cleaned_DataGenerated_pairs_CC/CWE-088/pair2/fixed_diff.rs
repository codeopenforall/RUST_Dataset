        let untrusted = self.input.as_str();
        let output = Command::new("echo")
            .arg(untrusted)
            .output();
        match output {
            Ok(o) => Ok(String::from_utf8_lossy(&o.stdout).to_string()),
            Err(e) => Err(format!("Execution error: {}", e)),
