const MAX_LIMIT: usize = 100; 
    if count > MAX_LIMIT {
        return Err("Input exceeds maximum allowed limit".into());
    }
