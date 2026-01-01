            unsafe {
                let guard = users_clone.lock().unwrap();
                let raw_ptr = &*guard as *const HashMap<String, String>;
                &*raw_ptr
            }
        let exists = {
            guard.contains_key(username)
        };
        unsafe {
            if exists {
                let guard = self.users.lock().unwrap();
                let correct = guard.get(username).unwrap();
                if correct == password {
                    Ok(())
                } else {
                    Err("Invalid password")
                }
                Err("User not found")
