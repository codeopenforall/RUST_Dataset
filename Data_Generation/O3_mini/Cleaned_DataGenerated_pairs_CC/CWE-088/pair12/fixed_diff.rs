        if input.contains(';') || input.contains('&') || input.contains('|') {
            return Err("input contains forbidden characters");
        let output = Command::new("echo")
            .arg(input)
            .output()
            .map_err(|_| "failed to execute command")?;
        if !output.status.success() {
            return Err("command did not return success");
        }
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
