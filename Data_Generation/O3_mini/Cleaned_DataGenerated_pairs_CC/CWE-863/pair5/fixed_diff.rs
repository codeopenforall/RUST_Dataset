const MAX_WORKERS: usize = 100; 
    let count = if count > MAX_WORKERS { MAX_WORKERS } else { count };
    let resource = Arc::new(Mutex::new(vec![0u8; 1]));
    let mut handles = Vec::with_capacity(count);
        handles.push(thread::spawn(move || {
            let mut guard = res.lock().unwrap();
            guard[0] = 42; 
        }));
    }
    for handle in handles {
        let _ = handle.join();
                    .unwrap_or(50);
    println!("Spawned {} workers (capped)", count);
