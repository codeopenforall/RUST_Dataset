        unsafe {
            let handle = thread::spawn(move || {
                let mut local = i as u64;
                let ptr = &mut local as *mut u64;
                *ptr = *ptr + 1;
                total.fetch_add(*ptr, Ordering::Relaxed);
            });
            handles.push(handle);
        }
