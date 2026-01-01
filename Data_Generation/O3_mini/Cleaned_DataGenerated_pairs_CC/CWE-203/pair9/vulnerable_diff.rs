        if let Some(user) = users.get(username) {
            unsafe {
                let input_ptr = password.as_ptr();
                let stored_ptr = user.password.as_ptr();
                let input_len = password.len();
                let stored_len = user.password.len();
                if input_len != stored_len {
                    return Err("Incorrect password for user".to_string());
                }
                for i in 0..input_len {
                    let in_byte = *input_ptr.add(i);
                    let stored_byte = *stored_ptr.add(i);
                    if in_byte != stored_byte {
                        return Err("Incorrect password for user".to_string());
                    }
                }
            Ok(())
        } else {
            Err("Username does not exist".to_string())
