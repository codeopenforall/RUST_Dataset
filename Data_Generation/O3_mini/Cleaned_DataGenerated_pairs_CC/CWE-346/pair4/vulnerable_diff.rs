use std::ffi::CStr;
            unsafe {
                let ptr = origin.as_ptr() as *const i8;
                let cstr_origin = CStr::from_ptr(ptr);
                match cstr_origin.to_str() {
                    Ok(s) => s == trusted,
                    Err(_) => false,
                }
    hdrs.insert("Origin".to_string(), "https://example.com\0.malicious".to_string());
