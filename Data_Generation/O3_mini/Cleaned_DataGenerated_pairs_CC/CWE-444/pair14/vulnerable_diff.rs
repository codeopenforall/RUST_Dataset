unsafe fn parse_raw(ptr: *const u8, len: usize) -> String {
    let slice = std::slice::from_raw_parts(ptr, len);
    String::from_utf8_lossy(slice).into_owned()
}
        let clen: usize = cl_val.parse().unwrap_or(0);
        let ptr = input.as_ptr().wrapping_add(header_end);
        if req.headers.contains_key("Transfer-Encoding") {
            unsafe {
                req.body = parse_raw(ptr, clen);
            }
        } else {
            req.body = input[header_end..header_end+clen].to_string();
        }
            unsafe {
                let chunk_ptr = input.as_ptr().add(pos);
                body.push_str(&parse_raw(chunk_ptr, chunk_size));
            }
        unsafe {
            let raw_ptr = &shared_clone as *const _ as *mut AtomicUsize;
            (*raw_ptr).store(1, Ordering::Relaxed);
        }
        "Transfer-Encoding: chunked\r\n",
