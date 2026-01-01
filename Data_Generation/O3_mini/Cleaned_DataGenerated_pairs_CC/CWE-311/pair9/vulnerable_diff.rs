        unsafe {
            let ptr = self.secret.as_ptr();
            let len = self.secret.len();
            std::slice::from_raw_parts(ptr, len).to_vec()
        }
        String::from_utf8(bytes).unwrap()
