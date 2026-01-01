    Failure, 
    fn constant_time_eq(a: &str, b: &str) -> bool {
        if a.len() != b.len() {
            return false;
        }
        let mut diff = 0;
        for (x, y) in a.as_bytes().iter().zip(b.as_bytes().iter()) {
            diff |= x ^ y;
        }
        diff == 0
    }
        let auth_success = if let Some(stored_pass) = stored {
            Self::constant_time_eq(stored_pass, password)
            Self::constant_time_eq("dummy_string", password)
        };
        if auth_success {
            Ok(())
        } else {
            Err(AuthError::Failure)
