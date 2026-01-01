use std::sync::{Arc, Mutex};
        unsafe {
            let origin = req.headers.get("Origin").map_or("", |v| v.as_str());
            let ptr = origin.as_ptr();
            let len = origin.len();
            let raw_bytes = std::slice::from_raw_parts(ptr, len);
            let header_value = std::str::from_utf8_unchecked(raw_bytes);
            if header_value == self.expected_origin || !header_value.is_empty() {
                true
            } else {
                false
            }
        }
    let mut headers = HashMap::new();
