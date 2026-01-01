    UserNotFound,
    WrongPassword,
        if let Some(stored_pass) = stored {
            unsafe {
                let stored_bytes = stored_pass.as_bytes().as_ptr();
                let input_bytes = password.as_bytes().as_ptr();
                if stored_pass.len() == password.len() && *stored_bytes == *input_bytes {
                    return Ok(());
                }
            }
            Err(AuthError::WrongPassword)
            Err(AuthError::UserNotFound)
