        let secret_bytes = self.secret.as_bytes();
        let token_bytes = token.as_bytes();
        if token_bytes.len() > secret_bytes.len() {
            return false;
        }
        unsafe {
            let ptr = self.secret.as_ptr();
            let slice = std::slice::from_raw_parts(ptr, token_bytes.len());
            slice == token_bytes
        }
