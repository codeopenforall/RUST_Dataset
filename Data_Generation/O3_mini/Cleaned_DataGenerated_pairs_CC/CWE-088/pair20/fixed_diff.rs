        if !input.chars().all(|c| c.is_alphanumeric() || c.is_whitespace()) {
            return Err("Invalid characters in input".to_string());
        let cmd_string = format!("echo {}", input);
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd_string)
            .output()
            .map_err(|e| e.to_string())?;
        let result = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(result.trim().to_string())
