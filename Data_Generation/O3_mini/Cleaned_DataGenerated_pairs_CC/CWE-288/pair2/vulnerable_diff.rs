static mut GLOBAL_DEBUG: bool = true; 
        if req.endpoint == "/admin" {
        }
        else if req.endpoint == "/debug" {
            unsafe {
                if GLOBAL_DEBUG {
                    return true;
                }
            }
            return req.token.as_ref() == Some(&self.secret);
        token: None,
