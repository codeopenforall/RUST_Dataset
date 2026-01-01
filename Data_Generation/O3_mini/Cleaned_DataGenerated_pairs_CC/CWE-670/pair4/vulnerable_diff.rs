use std::thread;
        let mut result: u32 = 0;
                result = *cnt;
                unsafe {
                    let ptr: *mut u32 = &mut result as *mut u32;
                    *ptr = 100;
                }
                let counter_clone = Arc::clone(&self.counter);
                let handle = thread::spawn(move || {
                    let mut lock = counter_clone.lock().unwrap();
                    if *lock >= 0 {
                        *lock += 10;
                    }
                });
                handle.join().unwrap();
                result = 100;
            _ => return Err("unsupported command"),
        Ok(result)
