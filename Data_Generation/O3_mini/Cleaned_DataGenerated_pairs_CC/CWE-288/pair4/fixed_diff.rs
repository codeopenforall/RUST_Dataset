    let enforce_secure = match env::var("APP_MODE") {
        Ok(val) if val == "production" => true,
        _ => false,
    };
    if enforce_secure {
        return secure_verify(acc);
    let config = Arc::new(Mutex::new(env::var("APP_MODE").unwrap_or_else(|_| "development".to_string())));
    let config_clone = Arc::clone(&config);
        let mode = config_clone.lock().unwrap();
        println!("Running in {} mode", *mode);
