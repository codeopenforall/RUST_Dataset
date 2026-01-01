        thread::sleep(Duration::from_millis(10));
        if guard.secret != input {
            return Err(format!(
                "Access denied for '{}'. Invalid credentials supplied.",
                guard.username
            ));
        Err(err) => panic!("{}", err),
