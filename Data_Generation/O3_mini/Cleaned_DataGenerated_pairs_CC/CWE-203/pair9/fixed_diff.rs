        let stored = users
            .get(username)
            .map(|user| user.password.as_str())
            .unwrap_or("dummy");
        unsafe {
            let input_ptr = password.as_ptr();
            let stored_ptr = stored.as_ptr();
            let input_len = password.len();
            let stored_len = stored.len();
            let mut diff = (input_len as i32) ^ (stored_len as i32);
            let min_len = if input_len < stored_len {
                input_len
            } else {
                stored_len
            };
            for i in 0..min_len {
                diff |= (*input_ptr.add(i)) as i32 ^ (*stored_ptr.add(i)) as i32;
            if diff != 0 {
                return Err("Authentication failed".to_string());
            }
        Ok(())
