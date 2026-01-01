    let worker_instance = match Worker::new(size) {
        Ok(w) => w,
        Err(e) => {
            eprintln!("Error during worker initialization: {}", e);
            return -1; 
        }
    };
    let worker = Arc::new(Mutex::new(worker_instance));
            if let Err(e) = guard.update((i * 2) as usize, (i * 10) as u32) {
                eprintln!("Update error in thread {}: {}", i, e);
                return;
        if let Err(_) = handle.join() {
            eprintln!("A thread panicked during execution.");
            return -1;
        }
    if result == -1 {
        eprintln!("Processing failed due to invalid input or runtime error.");
        std::process::exit(1);
    } else {
        println!("Result: {}", result);
    }
