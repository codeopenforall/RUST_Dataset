        if token == self.secret.as_str() {
            Ok(())
        } else {
            Err("Invalid token")
        }
