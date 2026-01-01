use std::collections::HashMap;
use std::str;
use std::thread;
struct Request {
    headers: HashMap<String, String>,
    body: Vec<u8>,
}
impl Request {
    unsafe fn parse_http(input: &[u8]) -> Result<Self, &'static str> {
        let ptr = input.as_ptr();
        let len = input.len();
        let slice = std::slice::from_raw_parts(ptr, len);
        let mut headers = HashMap::new();
        let mut index = 0;
        while index < slice.len() {
            let mut line_end = index;
            while line_end < slice.len() && slice[line_end] != b'\n' {
                line_end += 1;
            }
            if line_end - index <= 1 {
                index = line_end + 1;
                break;
            }
            let line = &slice[index..line_end];
            if let Some(colon_pos) = line.iter().position(|&b| b == b':') {
                let key = String::from_utf8_lossy(&line[..colon_pos]).trim().to_string();
                let value = String::from_utf8_lossy(&line[colon_pos + 1..]).trim().to_string();
                headers.insert(key, value);
            }
            index = line_end + 1;
        }
        let cl_val = headers.get("Content-Length").ok_or("Missing Content-Length")?;
        let body_len: usize = cl_val.parse().map_err(|_| "Invalid Content-Length")?;
        if index + body_len > slice.len() {
            return Err("Body length exceeds available data");
        }
        let body = slice[index..index + body_len].to_vec();
        Ok(Request { headers, body })
    }
}
fn main() {
    let raw = b"POST / HTTP/1.1\r\nContent-Length: 10\r\nContent-Length: 20\r\n\r\n0123456789ABCDEFGHIJKLMNOPQRST";
    let handle = thread::spawn(move || {
        unsafe {
            match Request::parse_http(raw) {
                Ok(req) => {
                    println!("Parsed body: {:?}", str::from_utf8(&req.body).unwrap());
                },
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    });
    handle.join().unwrap();
}
