        match req.headers.get("Authorization") {
            Some(header) => {
                let stored = self.token.lock().unwrap();
                &*stored == header
            },
            None => false,
            hdrs.insert("Authorization".to_string(), "secret".to_string());
