        unsafe {
            let key_ptr = config_clone.key.as_ptr();
            let len = config_clone.key.len();
            let key_slice = std::slice::from_raw_parts(key_ptr, len);
            String::from_utf8_lossy(key_slice).to_string()
        }
