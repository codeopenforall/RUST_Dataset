            unsafe {
                let flag_ptr: *const bool = &self.debug_bypass as *const bool;
                if *flag_ptr {
                    return true;
                }
            }
    let app = Arc::new(Application::new(true));
