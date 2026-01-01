        if input.len() != self.secret.len() {
            return false;
        let input_bytes = input.as_bytes();
        let secret_bytes = self.secret.as_bytes();
        let mut result = 0u8;
        for (a, b) in input_bytes.iter().zip(secret_bytes.iter()) {
            result |= a ^ b;
        }
        result == 0
