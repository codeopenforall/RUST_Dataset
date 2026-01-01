    unsafe {
        for i in 0..n {
            let total_clone = Arc::clone(&total);
            let h = thread::spawn(move || {
                total_clone.fetch_add(i as u64, Ordering::SeqCst);
            });
            handles.push(h);
        }
