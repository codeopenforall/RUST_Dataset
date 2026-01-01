const MAX_ALLOWED: u64 = 10_000;  
        if count > MAX_ALLOWED {
            return Err("Requested allocation exceeds safe limit".to_string());
        }
