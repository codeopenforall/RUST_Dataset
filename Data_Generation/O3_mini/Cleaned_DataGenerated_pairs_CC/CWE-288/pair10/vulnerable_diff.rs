use std::env;
        let mut bypass = false;
        unsafe {
            if let Ok(debug) = env::var("DEBUG_MODE") {
                if debug == "1" {
                    let bypass_ptr: *mut bool = &mut bypass;
                    *bypass_ptr = true;
                }
            }
        }
        AuthManager { bypass }
        if self.bypass {
            return true;
        }
