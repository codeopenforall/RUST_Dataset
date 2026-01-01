//////////////////////////////////////////////////////////////
// Secure version of the HTTP server processing logic.
// This revision fixes the vulnerability by verifying that if multiple
// "Content-Length" headers are present, all values must be identical.
// Safe Rust APIs are used for parsing, and unsafe blocks are removed.
//////////////////////////////////////////////////////////////

mod server {
    use std::sync::Arc;
    use std::thread;

    #[derive(Debug)]
    pub struct Request {
        pub header: String,
        pub body: Vec<u8>,
    }

    impl Request {
        // Parse the HTTP request in safe Rust.
        pub fn parse(raw: &[u8]) -> Result<Request, &'static str> {
            let text = std::str::from_utf8(raw).map_err(|_| "Invalid UTF8 in request")?;
            let parts: Vec<&str> = text.split("\r\n\r\n").collect();
            if parts.len() < 2 {
                return Err("Malformed HTTP request");
            }
            let header = parts[0].to_string();
            let body = parts[1].as_bytes().to_vec();
            Ok(Request { header, body })
        }
    }

    // Processes the HTTP request safely.
    // Fix: When duplicate "Content-Length" headers are found, all must agree.
    // If inconsistent values are found, the request is rejected.
    pub fn process_request(raw: &[u8]) -> Result<usize, &'static str> {
        let req = Request::parse(raw)?;
        let mut cl_values = Vec::new();
        for line in req.header.lines() {
            if line.to_lowercase().starts_with("content-length:") {
                let value_part = line.split(':').nth(1).ok_or("Missing header value")?;
                let trimmed = value_part.trim();
                let val = trimmed.parse::<usize>().map_err(|_| "Invalid number")?;
                cl_values.push(val);
            }
        }
        if cl_values.is_empty() {
            return Err("Missing Content-Length header");
        }
        // Ensure all Content-Length header values are the same.
        let first = cl_values[0];
        if !cl_values.iter().all(|&v| v == first) {
            return Err("Inconsistent Content-Length headers");
        }
        if first != req.body.len() {
            return Err("Body length does not match Content-Length");
        }
        Ok(first)
    }

    // Starts the processing in a separate thread.
    pub fn start(data: Vec<u8>) {
        let arc_data = Arc::new(data);
        let thread_data = arc_data.clone();
        let handle = thread::spawn(move || {
            let res = process_request(&thread_data);
            match res {
                Ok(len) => println!("Processed body. Confirmed Content-Length: {}", len),
                Err(e) => println!("Error: {}", e),
            }
        });
        handle.join().unwrap();
    }
}

fn main() {
    // A well-formed HTTP request with one Content-Length header.
    // Alternatively, if duplicate headers are present, they must agree.
    let req = b"POST / HTTP/1.1\r\nHost: example.com\r\nContent-Length: 11\r\n\r\nhello world";
    server::start(req.to_vec());
}