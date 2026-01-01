use std::ffi::CStr;
use std::os::raw::c_char;
        let header_value_ptr = match req.headers.get("Origin") {
            Some(value) => value.as_ptr() as *const c_char,
        let origin = unsafe {
            CStr::from_ptr(header_value_ptr).to_str().unwrap_or("")
        };
        if origin == self.allowed_origin {
            true
            h.insert("Origin".to_string(), "https://malicious.example.com".to_string());
            unsafe {
                println!("Request processed unsafely.");
            }
