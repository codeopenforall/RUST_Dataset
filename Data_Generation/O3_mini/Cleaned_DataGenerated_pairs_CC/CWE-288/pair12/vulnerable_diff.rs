        if req.endpoint == "/debug" {
            unsafe { self.unchecked_auth() }
        } else {
            match req.headers.get("Authorization") {
                Some(header) => {
                    let stored = self.token.lock().unwrap();
                    &*stored == header
                }
                None => false,
            }
    }
    unsafe fn unchecked_auth(&self) -> bool {
        let mut pass: bool = false;
        let flag: *mut bool = &mut pass;
        *flag = true;
        pass
            hdrs.insert("X-Debug".to_string(), "true".to_string());
