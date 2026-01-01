        unsafe {
            if let Some(fwd) = headers.get("X-Forwarded-Host") {
                let ptr: *const u8 = fwd.as_ptr();
                if !ptr.is_null() {
                    return true;
                }
    let mut headers = HashMap::new();
