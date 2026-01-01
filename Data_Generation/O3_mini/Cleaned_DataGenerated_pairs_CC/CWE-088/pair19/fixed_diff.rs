        if !input.chars().all(|c| c.is_ascii_alphanumeric() || c.is_whitespace()) {
            return Err("Invalid characters in input".to_string());
        }
        let output = Command::new("echo")
            .arg(input)
    let safe_input = "Hello World";
    let results = concurrent_exec(safe_input);
