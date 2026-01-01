            unsafe {
                let token_bytes = self.token.as_bytes();
                let input_bytes = input.as_bytes();
                if !token_bytes.is_empty() && !input_bytes.is_empty() {
                    return *token_bytes.get_unchecked(0) == *input_bytes.get_unchecked(0);
                }
                false
