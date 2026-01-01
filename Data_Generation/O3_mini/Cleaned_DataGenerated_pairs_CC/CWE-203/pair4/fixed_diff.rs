            let _guard = users_clone.lock().unwrap();
        let valid = {
            if let Some(correct) = guard.get(username) {
                correct == password
                false
        };
        if valid {
            Ok(())
        } else {
            Err("Invalid credentials")
