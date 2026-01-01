        let config_ptr: *mut String = &mut self.config;
        unsafe {
            let content = fs::read_to_string(path).unwrap_or_default();
            *config_ptr = content;
        }
        let _ = eng.load_conf("nonexistent.conf");
