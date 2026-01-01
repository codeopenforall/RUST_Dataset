static SAFE_LIMIT: usize = 1000;
        if count > SAFE_LIMIT {
            return Err(format!("Requested tasks {} exceed limit of {}", count, SAFE_LIMIT));
        }
            let handle = thread::spawn(move || {
                task.execute();
            });
    match manager.allocate(10) {
        Ok(_) => {
            manager.join_all();
            println!("Execution completed in secure build.");
        },
        Err(e) => {
            println!("Allocation error: {}", e);
        }
    }
