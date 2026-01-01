        const MAX_ITEMS: usize = 1000;
        if data.len() > MAX_ITEMS {
            return Err("Input size exceeds allowable limit");
        }
        if data.len() > 10 {
            let mut threads = Vec::with_capacity(data.len());
            for &item in data {
                let handle = thread::spawn(move || {
                    unsafe {
                        let mut val = item;
                        let ptr = &mut val as *mut u64;
                        *ptr = *ptr * 2;
                        *ptr
                    }
                });
                threads.push(handle);
            }
            for th in threads {
                result = result.saturating_add(th.join().map_err(|_| "Thread error")?);
            }
        } else {
            for &item in data {
                let doubled = unsafe {
                };
                result = result.saturating_add(doubled);
            }
