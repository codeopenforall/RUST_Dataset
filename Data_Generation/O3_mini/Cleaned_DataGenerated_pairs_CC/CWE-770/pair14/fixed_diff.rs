const MAX_THREADS: usize = 10; 
    if count > MAX_THREADS {
        return Err("Too many threads requested, potential out-of-bound access prevented".to_string());
    }
        Ok(_) => println!("Allocation safely completed with count {}", count),
