//////////////////////////////////////////////////////////////
// Realistic simulation of an HTTP server processing logic.
// This version contains a vulnerability resembling CWE-444,
// where inconsistent interpretation of duplicate HTTP headers
// may allow request smuggling, due to unsafely ignoring duplicate
// "Content-Length" header values and using an unsafe block.
//////////////////////////////////////////////////////////////

mod server {
    use std::sync::Arc;
    use std::thread;

    pub struct Request<'a> {
        pub header: &'a str,
        pub body: &'a [u8],
    }

    impl<'a> Request<'a> {
        // UNSAFE: Direct pointer arithmetic to split header and body.
        // This function does not validate duplicate header values.
        pub unsafe fn parse(raw: &'a [u8]) -> Request<'a> {
            // Find the header terminator "\r\n\r\n"
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
            }
            let header_ptr = raw.as_ptr();
            let header_slice = std::slice::from_raw_parts(header_ptr, pos);
            let header_str = std::str::from_utf8_unchecked(header_slice);
            let body = &raw[pos + 4..];
            Request {
                header: header_str,
                body,
            }
        }
    }

    // Processes the HTTP request and returns the Content-Length value.
    // Vulnerable: When duplicate "Content-Length" headers are present,
    // it simply takes the first header it finds without checking if the
    // values are consistent. This may allow an attacker to smuggle an extra
    // part of the request.
    pub unsafe fn process_request(raw: &[u8]) -> Result<usize, &'static str> {
        let req = Request::parse(raw);
        let lines: Vec<&str> = req.header.split("\r\n").collect();
        // Vulnerability: Only the first encountered header is used.
        let mut cl_val_option: Option<usize> = None;
        for line in lines {
            if line.to_lowercase().starts_with("content-length:") {
                let parts: Vec<&str> = line.split(':').collect();
                if let Some(val) = parts.get(1) {
                    let trimmed = val.trim();
                    if let Ok(n) = trimmed.parse::<usize>() {
                        if cl_val_option.is_none() {
                            cl_val_option = Some(n);
                        }
                        // Ignore additional values.
                    }
                }
            }
        }
        let cl_val = cl_val_option.unwrap_or(0);
        // Vulnerability: Even if the actual body is longer than cl_val
        // (as a result of duplicated header abuse), it returns success.
        if req.body.len() < cl_val {
            return Err("Body too short");
        }
        Ok(cl_val)
    }

    // Starts the processing in a separate thread.
    pub fn start(data: Vec<u8>) {
        let arc_data = Arc::new(data);
        let thread_data = arc_data.clone();
        let handle = thread::spawn(move || unsafe {
            let res = process_request(&thread_data);
            match res {
                Ok(len) => println!("Processed body. Interpreted Content-Length: {}", len),
                Err(e) => println!("Error: {}", e),
            }
        });
        handle.join().unwrap();
    }
}

fn main() {
    // Malicious request with duplicate Content-Length headers:
    // The first header "5" is taken, even though the actual body is longer.
    let req = b"POST / HTTP/1.1\r\nHost: example.com\r\nContent-Length: 5\r\nContent-Length: 10\r\n\r\nhello world";
    server::start(req.to_vec());
}