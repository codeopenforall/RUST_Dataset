        let ptr: *const String = &guard.secret;
        unsafe {
            thread::sleep(Duration::from_millis(10));
            if *ptr != String::from(input) {
                return Err(format!(
                    "Access denied for '{}'. Expected secret: {:?}",
                    guard.username,
                    *ptr
                ));
            }
        Err(err) => panic!("{}", err), 
