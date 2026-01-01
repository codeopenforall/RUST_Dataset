use std::collections::HashMap;
use std::str;
struct Request {
    headers: HashMap<String, String>,
    body: String,
}
impl Request {
    unsafe fn parse_headers(input: &str) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        for line in input.split("\r\n") {
            if let Some(idx) = line.find(':') {
                let key = line[..idx].trim();
                let value = line[idx + 1..].trim();
                let k = String::from_utf8_unchecked(key.as_bytes().to_vec());
                let v = String::from_utf8_unchecked(value.as_bytes().to_vec());
                headers.insert(k, v);
            }
        }
        headers
    }
    fn parse(raw: &str) -> Option<Self> {
        let parts: Vec<&str> = raw.split("\r\n\r\n").collect();
        if parts.len() < 2 {
            return None;
        }
        let header_part = parts[0];
        let body_part = parts[1];
        let headers = unsafe { Self::parse_headers(header_part) };
        let body = if let Some(cl_val) = headers.get("Content-Length") {
            if let Ok(cl) = cl_val.parse::<usize>() {
                if body_part.len() >= cl {
                    body_part[..cl].to_string()
                } else {
                    String::new()
                }
            } else {
                String::new()
            }
        } else if headers.get("Transfer-Encoding")
                        .map(|v| v.to_lowercase() == "chunked")
                        .unwrap_or(false) {
            Request::decode_chunked(body_part)
        } else {
            body_part.to_string()
        };
        Some(Request { headers, body })
    }
    fn decode_chunked(input: &str) -> String {
        let mut res = String::new();
        let mut lines = input.lines();
        loop {
            let size_line = match lines.next() {
                Some(line) => line,
                None => break,
            };
            let chunk_size = match usize::from_str_radix(size_line, 16) {
                Ok(0) => break,
                Ok(n) => n,
                Err(_) => break,
            };
            let mut chunk = String::new();
            let mut read_bytes = 0;
            while read_bytes < chunk_size {
                if let Some(line) = lines.next() {
                    let len = line.len();
                    chunk.push_str(line);
                    read_bytes += len;
                } else {
                    break;
                }
            }
            res.push_str(&chunk);
        }
        res
    }
}
fn main() {
    let input = "POST / HTTP/1.1\r\nHost: example.com\r\nContent-Length: 5\r\nTransfer-Encoding: chunked\r\n\r\n7\r\nPayloadData\r\n0\r\n\r\n";
    if let Some(req) = Request::parse(input) {
        println!("Processed body length: {}", req.body.len());
    } else {
        println!("Failed to parse request");
    }
}
