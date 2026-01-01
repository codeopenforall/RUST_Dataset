        thread::sleep(Duration::from_millis(100));
        let lock_b = b2.lock().unwrap();
        thread::sleep(Duration::from_millis(100));
