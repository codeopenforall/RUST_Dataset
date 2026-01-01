    let content = fs::read_to_string("setting.conf")
        .map_err(|e| format!("Failed to read configuration: {}", e))?;
    let threshold = content
        .trim()
        .parse::<u32>()
        .map_err(|e| format!("Invalid threshold value: {}", e))?;
    if threshold == 0 {
        return Err("Threshold cannot be zero".to_string());
    }
        let sys = configure().expect("System configuration failure");
