    let content = fs::read_to_string("setting.conf").unwrap_or_default();
    let threshold = content.trim().parse::<u32>().unwrap_or_default();
        let sys = configure().expect("Failed to initialize system");
