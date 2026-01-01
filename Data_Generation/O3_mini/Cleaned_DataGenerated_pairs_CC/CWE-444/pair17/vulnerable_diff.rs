}
unsafe fn unsafe_str<'a>(ptr: *const u8, len: usize) -> &'a str {
    std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, len))
    let header_ptr = header_str.as_ptr();
    let header_len = header_str.len();
    let body_start = header_ptr as usize + header_len + 4;
    let total_ptr = input.as_ptr();
    let total_len = input.len();
    let body_len = if let Some(cl) = headers.get("Content-Length") {
        cl.parse::<usize>().unwrap_or(0)
        total_len - (body_start - total_ptr as usize)
    };
    let body = unsafe {
        let body_slice = std::slice::from_raw_parts(body_start as *const u8, body_len);
        body_slice.to_vec()
