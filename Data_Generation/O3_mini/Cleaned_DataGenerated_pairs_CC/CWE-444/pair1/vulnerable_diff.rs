    pub struct Request<'a> {
        pub header: &'a str,
        pub body: &'a [u8],
    impl<'a> Request<'a> {
        pub unsafe fn parse(raw: &'a [u8]) -> Request<'a> {
            let mut pos = 0;
            while pos <= raw.len().saturating_sub(4) {
                if raw[pos] == b'\r'
                    && raw[pos + 1] == b'\n'
                    && raw[pos + 2] == b'\r'
                    && raw[pos + 3] == b'\n'
                {
                    break;
                }
                pos += 1;
            let header_ptr = raw.as_ptr();
            let header_slice = std::slice::from_raw_parts(header_ptr, pos);
            let header_str = std::str::from_utf8_unchecked(header_slice);
            let body = &raw[pos + 4..];
            Request {
                header: header_str,
                body,
            }
    pub unsafe fn process_request(raw: &[u8]) -> Result<usize, &'static str> {
        let req = Request::parse(raw);
        let lines: Vec<&str> = req.header.split("\r\n").collect();
        let mut cl_val_option: Option<usize> = None;
        for line in lines {
                let parts: Vec<&str> = line.split(':').collect();
                if let Some(val) = parts.get(1) {
                    let trimmed = val.trim();
                    if let Ok(n) = trimmed.parse::<usize>() {
                        if cl_val_option.is_none() {
                            cl_val_option = Some(n);
                        }
                    }
                }
        let cl_val = cl_val_option.unwrap_or(0);
        if req.body.len() < cl_val {
            return Err("Body too short");
        Ok(cl_val)
        let handle = thread::spawn(move || unsafe {
                Ok(len) => println!("Processed body. Interpreted Content-Length: {}", len),
    let req = b"POST / HTTP/1.1\r\nHost: example.com\r\nContent-Length: 5\r\nContent-Length: 10\r\n\r\nhello world";
