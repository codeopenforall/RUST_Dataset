const MAX_MEMORY_ALLOCATION: usize = 100_000; 
const MAX_THREAD_COUNT: usize = 500; 
        if count > MAX_MEMORY_ALLOCATION {
            return Err("Requested resources exceed safe limit".to_string());
        }
        let safe_count = if count > MAX_THREAD_COUNT { MAX_THREAD_COUNT } else { count };
        for i in 0..safe_count {
    let resources = manager.create_resources(50_000).expect("Allocation within limit");
    println!("Safely allocated {} bytes", resources.len());
    manager.spawn_workers(600); 
