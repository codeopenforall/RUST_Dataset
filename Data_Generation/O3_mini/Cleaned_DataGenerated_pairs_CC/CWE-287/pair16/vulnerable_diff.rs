        unsafe {
            if user_token.is_empty() {
                return false;
            }
            let expected_ptr = self.token.as_ptr();
            let user_ptr = user_token.as_ptr();
            *expected_ptr == *user_ptr
        }
