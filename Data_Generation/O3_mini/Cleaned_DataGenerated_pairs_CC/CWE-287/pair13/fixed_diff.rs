        let secret = self.secret.lock().unwrap();
        token == secret.as_str()
                let token_input = "s3cr3t";
