const MAX_ALLOC: usize = 100_000; 
        if n > MAX_ALLOC {
            return Err("Request exceeds maximum allowed allocation size");
        }
