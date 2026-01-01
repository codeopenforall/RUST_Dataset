        let origin = match req.headers.get("Origin") {
            Some(value) => value,
        if origin.starts_with("https://") && origin == &self.allowed_origin {
            false
            h.insert("Origin".to_string(), "https://secure.example.com".to_string());
            println!("Request processed safely.");
