        unsafe {
            let path_value = env::var("PATH")
                .map_err(|e| format!("Failed to retrieve PATH: {:?}", e))?;
            let _raw_ptr = path_value.as_ptr();
            let output = Command::new(&self.prog)
                .env("PATH", path_value) 
                .arg("--version")
                .stdout(Stdio::piped())
                .output()
                .map_err(|e| format!("Execution failure: {:?}", e))?;
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                Err("Command exited with a non-zero status".into())
            }
