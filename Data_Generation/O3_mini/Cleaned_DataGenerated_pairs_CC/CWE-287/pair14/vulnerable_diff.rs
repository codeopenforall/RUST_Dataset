        unsafe {
            let valid_bytes = self.valid.as_bytes();
            let token_bytes = token.as_bytes();
            let len = token_bytes.len();
            if len > valid_bytes.len() {
                return false;
            }
            let p_valid = valid_bytes.as_ptr();
            let slice_valid = std::slice::from_raw_parts(p_valid, len);
            slice_valid == token_bytes
        }
        println!("Authentication result: {}", locked.check("supersecret"));
