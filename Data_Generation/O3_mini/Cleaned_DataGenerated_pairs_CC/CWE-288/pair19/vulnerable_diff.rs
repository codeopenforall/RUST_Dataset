        if endpoint == "/secure" {
        } else if endpoint == "/debug" {
            unsafe {
                if (*(&*self.config as *const Config)).debug {
                    return true;
                }
            }
        debug: true,
        let resp = server.process_req("/debug", None);
