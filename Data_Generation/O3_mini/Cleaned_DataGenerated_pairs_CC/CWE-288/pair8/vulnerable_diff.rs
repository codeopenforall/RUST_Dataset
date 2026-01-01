        let bypass = unsafe {
            let p: *const bool = &self.debug_mode;
            *p
        };
        if path == "debug" && bypass {
            return true;
