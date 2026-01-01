            if self.token.len() != input.len() {
                return false;
            let mut diff = 0u8;
            let token_bytes = self.token.as_bytes();
            let input_bytes = input.as_bytes();
            for (&a, &b) in token_bytes.iter().zip(input_bytes.iter()) {
                diff |= a ^ b;
            }
            diff == 0
