    unsafe fn parse(input: &str) -> Self {
        let content_length: usize = headers.get("Content-Length")
        let src = input.as_ptr().add(body_offset); 
        let mut buffer: Vec<u8> = Vec::with_capacity(content_length);
        buffer.set_len(content_length);         
        std::ptr::copy_nonoverlapping(src, buffer.as_mut_ptr(), content_length); 
        let body = String::from_utf8_lossy(&buffer).into_owned();
    let req = unsafe { HttpRequest::parse(request_str) };
