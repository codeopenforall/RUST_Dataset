const MAX_THREADS: usize = 200; 
        if count > MAX_THREADS {
            return Err("Excessive resource request");
        }
