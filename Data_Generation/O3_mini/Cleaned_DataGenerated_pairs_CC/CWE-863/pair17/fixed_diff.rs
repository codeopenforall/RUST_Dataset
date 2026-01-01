const MAX_ALLOWED: usize = 1000;
    if n > MAX_ALLOWED {
        return Err("Input too large, maximum allowed is 1000");
    }
    for i in 0..n {
        let total_clone = Arc::clone(&total);
        let h = thread::spawn(move || {
            total_clone.fetch_add(i as u64, Ordering::SeqCst);
        });
        handles.push(h);
