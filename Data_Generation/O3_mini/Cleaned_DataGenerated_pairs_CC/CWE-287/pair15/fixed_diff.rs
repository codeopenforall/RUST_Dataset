        if input.len() != self.secret.len() {
            return false;
        self.secret.as_bytes() == input.as_bytes()
