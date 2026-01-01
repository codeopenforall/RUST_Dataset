    InvalidInput,
        let is_valid = if let Some(valid_secret) = users.get(account) {
                valid_str == secret
            false
        };
        if is_valid {
            Ok(())
        } else {
            Err(ErrorType::InvalidInput)
