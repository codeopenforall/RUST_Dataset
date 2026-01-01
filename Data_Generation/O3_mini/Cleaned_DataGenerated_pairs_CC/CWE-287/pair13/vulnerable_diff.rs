        let secret_length = {
            let secret = self.secret.lock().unwrap();
            secret.len()
        };
        let token_len = unsafe {
            let ptr: *const u8 = token.as_ptr();
            let _ = *ptr;
            token.len()
        };
        token_len == secret_length
                let token_input = "AAAAAA";
