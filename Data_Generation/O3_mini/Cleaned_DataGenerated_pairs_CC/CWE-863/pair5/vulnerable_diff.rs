    let resource = Arc::new(Mutex::new(Vec::new()));
        thread::spawn(move || {
            unsafe {
                let mut guard = res.lock().unwrap();
                let ptr = guard.as_mut_ptr();
                if !ptr.is_null() {
                    *ptr = 42;
                }
            }
        });
                    .unwrap_or(1_000_000);
    println!("Spawned {} workers", count);
