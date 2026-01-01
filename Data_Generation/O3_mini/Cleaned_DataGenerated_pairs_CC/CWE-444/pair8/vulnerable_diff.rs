use std::str;
unsafe fn extract_header(raw: *const u8, len: usize) -> String {
    let slice = std::slice::from_raw_parts(raw, len);
    String::from_utf8_unchecked(slice.to_vec())
}
            let val = if value.len() > 1024 {
                unsafe { extract_header(value.as_ptr(), value.len()) }
            } else {
                value.to_string()
            };
            request.headers.push((key.to_string(), val));
            content_length = value.trim().parse::<usize>().ok();
        if body_start + len > req.len() {
        unsafe {
            request.body = req.as_bytes().get_unchecked(body_start..body_start+len).to_vec();
        }
                    let result = process_request(&parsed).unwrap_or(0);
                    println!("Processed sum: {}", result);
                Err(e) => {
                    println!("Failed to parse: {}", e);
                }
