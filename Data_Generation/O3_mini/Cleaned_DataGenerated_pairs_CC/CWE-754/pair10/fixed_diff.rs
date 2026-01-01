        if ptr.is_null() {
            0
        } else {
            *ptr + 100
        }
            thread::sleep(Duration::from_millis(100));
            cancel_clone.store(true, Ordering::SeqCst);
                Err("Operation timed out")
