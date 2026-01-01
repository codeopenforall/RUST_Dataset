const MAX_ALLOWED: usize = 10;
        if count > MAX_ALLOWED {
            return Err(format!("Requested thread count {} exceeds the limit of {}", count, MAX_ALLOWED));
        }
