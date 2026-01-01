        unsafe {
            if input.is_empty() || (*self.secret).is_empty() {
                return false;
            }
            let input_ptr = input.as_ptr();
            let secret_ptr = self.secret.as_ptr();
            *input_ptr == *secret_ptr
