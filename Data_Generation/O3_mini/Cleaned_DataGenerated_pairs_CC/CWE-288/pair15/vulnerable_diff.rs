static mut GLOBAL_DEBUG: bool = true;
        unsafe {
            if GLOBAL_DEBUG && req.path == "/debug" {
                return true;
            }
        }
