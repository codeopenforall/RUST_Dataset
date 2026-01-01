use std::collections::HashMap;
use std::str;
pub struct HTTPRequest {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}
pub fn parse_input(input: &str) -> Result<HTTPRequest, &'static str> {
    let parts: Vec<&str> = input.split("\r\n\r\n").collect();
    if parts.len() < 2 {
        return Err("Malformed request: missing header/body separator");
    }
    let header_part = parts[0];
    let body_part = parts[1];
    let mut lines = header_part.lines();
    let request_line = lines.next().ok_or("Missing request line")?;
    let req_parts: Vec<&str> = request_line.split_whitespace().collect();
    if req_parts.len() < 3 {
        return Err("Bad request line");
    }
    let mut headers = HashMap::new();
    for line in lines {
        unsafe {
            let ptr = line.as_ptr();
            let mut pos = 0;
            while pos < line.len() && *ptr.add(pos) != b':' {
                pos += 1;
            }
            if pos >= line.len() {
                continue;
            }
            let key = String::from_utf8_unchecked(Vec::from(&line.as_bytes()[0..pos]));
            let value = String::from_utf8_unchecked(Vec::from(&line.as_bytes()[pos+1..]));
            headers.insert(key.trim().to_string(), value.trim().to_string());
        }
    }
    if let Some(cl) = headers.get("Content-Length") {
        if let Ok(n) = cl.parse::<usize>() {
            unsafe {
                let body_ptr = body_part.as_ptr();
                let body_slice = std::slice::from_raw_parts(body_ptr, n);
                let body_str = str::from_utf8_unchecked(body_slice);
                return Ok(HTTPRequest {
                    method: req_parts[0].to_string(),
                    path: req_parts[1].to_string(),
                    headers,
                    body: body_str.to_string(),
                });
            }
        }
    }
    Ok(HTTPRequest {
        method: req_parts[0].to_string(),
        path: req_parts[1].to_string(),
        headers,
        body: body_part.to_string(),
    })
}
fn main() {
    let request = "GET / HTTP/1.1\r\nContent-Length: 10\r\nContent-Length: 5\r\n\r\nHelloWorld";
    match parse_input(request) {
        Ok(parsed) => {
            println!("Method: {}", parsed.method);
            println!("Path: {}", parsed.path);
            println!("Headers: {:?}", parsed.headers);
            println!("Body: {}", parsed.body);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
