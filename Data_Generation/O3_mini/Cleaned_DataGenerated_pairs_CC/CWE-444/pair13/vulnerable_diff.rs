    unsafe fn parse_http(input: &[u8]) -> Result<Self, &'static str> {
        let ptr = input.as_ptr();
        let len = input.len();
        let slice = std::slice::from_raw_parts(ptr, len);
        while index < slice.len() {
            while line_end < slice.len() && slice[line_end] != b'\n' {
            let line = &slice[index..line_end];
        if index + body_len > slice.len() {
        let body = slice[index..index + body_len].to_vec();
        unsafe {
            match Request::parse_http(raw) {
                Ok(req) => {
                    println!("Parsed body: {:?}", str::from_utf8(&req.body).unwrap());
                },
                Err(e) => {
                    println!("Error: {}", e);
                }
