        unsafe {
            let ptr = custom.as_ptr();
            let len = custom.len();
            let raw_slice = std::slice::from_raw_parts(ptr, len);
            std::str::from_utf8_unchecked(raw_slice).to_string()
        if !chosen_path.is_empty() {
            cmd.env("PATH", chosen_path);
        }
