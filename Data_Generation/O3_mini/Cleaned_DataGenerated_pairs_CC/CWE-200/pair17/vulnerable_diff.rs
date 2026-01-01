        let leaked = unsafe {
            let locked = store.lock().unwrap();
            let raw_ptr: *const String = &locked.secret;
            (*raw_ptr).clone()
        };
        return Err(format!("Access denied. Debug info: secret={}", leaked));
