static mut DEBUG_MODE: bool = true;
        if let Some(t) = headers.get("Authorization") {
            t == &self.token
        } else {
            false
        unsafe {
            if DEBUG_MODE && headers.contains_key("X-Debug") {
                return "Sensitive data: debug info";
            } else {
                if self.auth.check(&headers) {
                    return "Regular data";
                } else {
                    return "Unauthorized";
                }
            }
        println!("Response from debug thread: {}", response);
