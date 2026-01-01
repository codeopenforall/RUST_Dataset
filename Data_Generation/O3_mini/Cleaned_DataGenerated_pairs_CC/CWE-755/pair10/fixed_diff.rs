    fn new() -> Result<Self, &'static str> {
        let config_content = load_config("config.txt")?; 
        let threshold = config_content.trim().parse::<u32>().map_err(|_| "Invalid threshold format")?;
        if threshold == 0 {
            return Err("Threshold must be greater than zero");
        }
        Ok(Handler { threshold })
    let instance = match Handler::new() {
        Ok(cfg) => cfg,
        Err(e) => {
            println!("Configuration error encountered: {}", e);
            return;
        }
    };
            let _ = arc_clone;
                println!("Operating with secure threshold: {}", conf.threshold);
