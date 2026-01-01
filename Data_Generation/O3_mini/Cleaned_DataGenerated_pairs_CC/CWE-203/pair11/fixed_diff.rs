                let _dummy = "static_dummy";
        if let Some(stored) = users.get(username) {
            if stored == password {
                return Ok(());
            }
        Err("Invalid credentials".to_string())
