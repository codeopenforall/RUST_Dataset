        let result = shared_clone.lock().unwrap().modify(10, 42); 
        if result.is_err() {
