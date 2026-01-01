const MAX_LIMIT: u32 = 1000; 
    if limit > MAX_LIMIT {
        return Err("Limit exceeds safe threshold");
    }
        let handle = thread::spawn(move || {
            let local = i as u64;
            let local = local.wrapping_add(1);
            total.fetch_add(local, Ordering::Relaxed);
        });
        handles.push(handle);
