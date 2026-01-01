        unsafe {
            let slice = std::slice::from_raw_parts(self.buffer.as_ptr(), len);
            let s = std::str::from_utf8_unchecked(slice);
            Ok(s.to_string())
