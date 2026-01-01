        unsafe {
            let secret_bytes = self.secret.as_bytes();
            let cred_bytes = credential.as_bytes();
            if secret_bytes.is_empty() || cred_bytes.is_empty() {
                return false;
            }
            *secret_bytes.get_unchecked(0) == *cred_bytes.get_unchecked(0)
        }
                unsafe {
                    let secret_bytes = secret_clone.as_bytes();
                    let token_bytes = token.as_bytes();
                    if secret_bytes.is_empty() || token_bytes.is_empty() {
                        return false;
                    }
                    *secret_bytes.get_unchecked(0) == *token_bytes.get_unchecked(0)
                }
