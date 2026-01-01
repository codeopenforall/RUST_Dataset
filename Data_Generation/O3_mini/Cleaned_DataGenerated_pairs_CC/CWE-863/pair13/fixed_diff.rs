    if user != "admin" && amount > MAX_LIMIT {
        return Err("Unauthorized request exceeds the permitted limit".to_string());
        ("admin", 2 * 1024 * 1024), 
