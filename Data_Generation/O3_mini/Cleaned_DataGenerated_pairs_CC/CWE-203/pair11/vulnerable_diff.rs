                return Err("User does not exist".to_string());
        let stored = users.get(username).unwrap();
        if stored != password {
            return Err("Incorrect password".to_string());
        Ok(())
