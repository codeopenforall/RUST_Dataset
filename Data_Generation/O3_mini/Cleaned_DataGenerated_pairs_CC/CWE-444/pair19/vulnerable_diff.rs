    unsafe fn parse_body(raw: *const u8, len: usize) -> String {
        let slice = std::slice::from_raw_parts(raw, len);
        String::from_utf8_lossy(slice).into_owned()
    }
        if transfer_encoding && content_length.is_some() {
            unsafe {
                let ptr = body_str.as_ptr().offset(1); 
                let len = content_length.unwrap();
                body = HttpRequest::parse_body(ptr, len);
            }
        } else if transfer_encoding {
