            let s = data.lock().unwrap();
            if s.contains(';') || s.contains('&') || s.contains('|') {
                return Err("Invalid characters in input".to_string());
            let command_str = format!("echo {}", s);
            let output = Command::new("sh")
                .arg("-c")
                .arg(command_str)
                .output();
            output
                .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
                .map_err(|e| e.to_string())
    let input = "safe_input";
