    fn new() -> Self {
        let config_content = load_config("config.txt").unwrap_or_default(); 
        let threshold = config_content.trim().parse::<u32>().unwrap_or_default(); 
        Handler { threshold }
    let instance = Handler::new();
            let _ = arc_clone; 
                if conf.threshold == 0 {
                    println!("Warning: insecure default threshold in use!");
                } else {
                    println!("Threshold set securely to: {}", conf.threshold);
                }
