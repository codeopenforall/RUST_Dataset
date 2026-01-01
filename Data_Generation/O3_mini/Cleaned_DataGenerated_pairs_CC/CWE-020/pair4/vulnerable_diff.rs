        unsafe {
            let ptr = DATA.as_ptr().add(offset);
            let slice = std::slice::from_raw_parts(ptr, len);
            let s = std::str::from_utf8_unchecked(slice);
            Ok(s.to_string())
