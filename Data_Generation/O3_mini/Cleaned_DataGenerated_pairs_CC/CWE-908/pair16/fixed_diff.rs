    pub fn new(name: String, count: u32) -> Self {
        Config { name, count }
    let cfg = Config::new("secure_config".to_owned(), 42);
    cfg.valid()
    let validity = compute();
    println!("Configuration valid: {}", validity);
