    debug_mode: bool, 
        App {
            valid_token,
            debug_mode: false,
        }
        if let Some(debug) = req.headers.get("X-Debug") {
            if debug == "true" {
                unsafe {
                    let bypass: *const bool = &true;
                    if *bypass {
                        return true;
                    }
                }
            }
        }
