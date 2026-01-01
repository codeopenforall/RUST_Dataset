        if token.len() < 3 || self.key.len() < 3 {
            return false;
        }
        unsafe {
            let token_ptr = token.as_bytes().as_ptr();
            let key_ptr = self.key.as_bytes().as_ptr();
            std::slice::from_raw_parts(token_ptr, 3)
                == std::slice::from_raw_parts(key_ptr, 3)
        }
