        let trusted_path = "/usr/bin:/bin";
        let output = Command::new(&self.prog)
            .env("PATH", trusted_path) 
            .arg("--version")
            .stdout(Stdio::piped())
            .output()
            .map_err(|e| format!("Execution failure: {:?}", e))?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Err("Command exited with a non-zero status".into())
