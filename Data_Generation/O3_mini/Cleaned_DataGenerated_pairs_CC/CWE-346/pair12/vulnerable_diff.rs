        let ptr = header.as_ptr();
        let len = header.len();
        let hdr = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, len)) };
        if hdr.contains(&self.allowed_origin) {
            true
            true 
