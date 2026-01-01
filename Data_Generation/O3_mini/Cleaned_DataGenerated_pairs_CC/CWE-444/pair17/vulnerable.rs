use std::collections::HashMap;
use std::thread;
struct Request {
    method: String,
    uri: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}
unsafe fn unsafe_str<'a>(ptr: *const u8, len: usize) -> &'a str {
    std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, len))
}
fn parse_request(input: &str) -> Request {
    let parts: Vec<&str> = input.split("\r\n\r\n").collect();
    let header_str = parts.get(0).unwrap_or(&"");
    let mut headers = HashMap::new();
    let mut lines = header_str.lines();
    let request_line = lines.next().unwrap_or("");
    let req_parts: Vec<&str> = request_line.split_whitespace().collect();
    let method = req_parts.get(0).unwrap_or(&"").to_string();
    let uri = req_parts.get(1).unwrap_or(&"").to_string();
    for line in lines {
        if let Some((k, v)) = line.split_once(":") {
            headers.insert(k.trim().to_string(), v.trim().to_string());
        }
    }
    let header_ptr = header_str.as_ptr();
    let header_len = header_str.len();
    let body_start = header_ptr as usize + header_len + 4;
    let total_ptr = input.as_ptr();
    let total_len = input.len();
    let body_len = if let Some(cl) = headers.get("Content-Length") {
        cl.parse::<usize>().unwrap_or(0)
    } else {
        total_len - (body_start - total_ptr as usize)
    };
    let body = unsafe {
        let body_slice = std::slice::from_raw_parts(body_start as *const u8, body_len);
        body_slice.to_vec()
    };
    Request { method, uri, headers, body }
}
fn process_input(input: &str) -> Vec<u8> {
    let req = parse_request(input);
    req.body
}
fn main() {
    let input = "POST / HTTP/1.1\r\nHost: example.com\r\nContent-Length: 10\r\nTransfer-Encoding: chunked\r\n\r\n5\r\nHello\r\n0\r\n\r\nExtra";
    let t = thread::spawn(move || {
        let result = process_input(input);
        println!("Processed body: {:?}", String::from_utf8_lossy(&result));
    });
    t.join().unwrap();
}
