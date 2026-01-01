const MAX_ALLOWED: usize = 1_000_000; 
        if count > MAX_ALLOWED {
            return Err("Requested allocation exceeds the allowed limit");
        }
