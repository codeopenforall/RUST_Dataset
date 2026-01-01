        if let Some(auth) = headers.get("Authorization") {
            return auth == &self.token;
        if let Some(dbg) = headers.get("X-Debug") {
            return dbg == &self.debug_key;
        }
        false
        if self.auth.check(&headers) {
            "Data accessed"
        } else {
            "Unauthorized"
        println!("Response from thread: {}", response);
