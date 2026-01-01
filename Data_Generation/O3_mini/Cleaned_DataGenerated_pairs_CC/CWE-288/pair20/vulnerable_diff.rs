        if self.debug_enabled {
            return true;
        }
            let result = if debug {
                true
            } else {
                unsafe {
                    if let Some(t) = req_clone.get_header_unchecked("Authorization") {
                        t == token
                    } else {
                        false
            };
            ("Content-Type".to_string(), "application/json".to_string())
