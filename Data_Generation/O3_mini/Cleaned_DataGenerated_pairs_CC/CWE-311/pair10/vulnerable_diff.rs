        unsafe {
            let raw_ptr = self.data.as_ptr();
            let len = self.data.len();
            let bytes = std::slice::from_raw_parts(raw_ptr, len);
            String::from_utf8_lossy(bytes).to_string()
        }
