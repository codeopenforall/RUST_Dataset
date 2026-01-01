const MAX_ALLOWED: usize = 10;
    if count > MAX_ALLOWED {
        return Err("Requested resources exceed the allowed limit".to_string());
    }
