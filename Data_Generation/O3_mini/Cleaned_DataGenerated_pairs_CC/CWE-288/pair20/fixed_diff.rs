                }
            }
        }
        if self.debug_enabled {
            unsafe {
                if let Some(origin) = req.get_header_unchecked("X-Origin") {
                    if origin == "localhost" {
                        return true;
                    }
            let mut result = false;
            unsafe {
                if let Some(token_val) = req_clone.get_header_unchecked("Authorization") {
                    if token_val == token {
                        result = true;
                if !result && debug {
                    if let Some(origin) = req_clone.get_header_unchecked("X-Origin") {
                        if origin == "localhost" {
                            result = true;
                        }
                    }
                }
            }
            ("Content-Type".to_string(), "application/json".to_string()),
            ("X-Origin".to_string(), "production_host".to_string())
