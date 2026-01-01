        let output = Command::new("echo")
            .arg(&self.data)
            .output();
        match output {
            Ok(out) => Ok(String::from_utf8_lossy(&out.stdout).into_owned()),
            Err(e) => Err(e.to_string()),
