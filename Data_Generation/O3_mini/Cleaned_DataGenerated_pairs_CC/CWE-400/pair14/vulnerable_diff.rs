        let mut threads = Vec::with_capacity(data.len());
        for &item in data {
            let handle = thread::spawn(move || {
                unsafe {
                }
            });
            threads.push(handle);
        }
        for th in threads {
            result = result.saturating_add(th.join().map_err(|_| "Thread error")?);
