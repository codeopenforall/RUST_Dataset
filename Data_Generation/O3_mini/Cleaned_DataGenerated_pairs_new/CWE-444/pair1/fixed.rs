mod server {
    use std::sync::Arc;
    use std::thread;
    #[derive(Debug)]
    pub struct Request {
        pub header: String,
        pub body: Vec<u8>,
    }
    impl Request {
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
        let first = cl_values[0];
        if !cl_values.iter().all(|&v| v == first) {
            return Err("Inconsistent Content-Length headers");
        }
        if first != req.body.len() {
            return Err("Body length does not match Content-Length");
        }
        Ok(first)
    }
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
    let req = b"POST / HTTP/1.1\r\nHost: example.com\r\nContent-Length: 11\r\n\r\nhello world";
    server::start(req.to_vec());
}
