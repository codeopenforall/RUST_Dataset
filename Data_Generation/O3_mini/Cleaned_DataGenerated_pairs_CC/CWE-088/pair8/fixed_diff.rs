        let output = Command::new("echo")
            .arg(&self.payload)
            .output();
        match output {
            Ok(o) => {
                let text = String::from_utf8_lossy(&o.stdout).into_owned();
                Ok(text)
            },
            Err(e) => Err(format!("Execution error: {}", e)),
