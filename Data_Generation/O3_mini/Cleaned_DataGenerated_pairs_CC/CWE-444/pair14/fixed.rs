use std::collections::HashMap;
use std::sync::atomic::Ordering;
struct Request {
    headers: HashMap<String, String>,
    body: String,
}
impl Request {
    fn new() -> Self {
        Request {
            headers: HashMap::new(),
            body: String::new(),
        }
    }
}
fn process_request(input: &str) -> Result<Request, &'static str> {
    let mut req = Request::new();
    let mut lines = input.split("\r\n");
    lines.next();
    for line in lines {
        if line.is_empty() { break; }
        if let Some((key, value)) = line.split_once(": ") {
            req.headers.insert(key.to_string(), value.to_string());
        }
    }
    if req.headers.contains_key("Content-Length") && req.headers.contains_key("Transfer-Encoding") {
        return Err("Ambiguous headers: both Content-Length and Transfer-Encoding are present");
    }
    if let Some(cl_val) = req.headers.get("Content-Length") {
        let clen: usize = cl_val.parse().map_err(|_| "Invalid Content-Length")?;
        let header_end = input.find("\r\n\r\n").ok_or("Malformed request")? + 4;
        if header_end + clen > input.len() {
            return Err("Incomplete body");
        }
        req.body = input[header_end..header_end+clen].to_string();
    } else if req.headers.contains_key("Transfer-Encoding") {
        let mut body = String::new();
        let mut pos = input.find("\r\n\r\n").ok_or("Malformed request")? + 4;
        loop {
            let pos_newline = input[pos..].find("\r\n").ok_or("Malformed chunk")? + pos;
            let chunk_size_str = &input[pos..pos_newline];
            let chunk_size = usize::from_str_radix(chunk_size_str, 16)
                .map_err(|_| "Invalid chunk size")?;
            if chunk_size == 0 { break; }
            pos = pos_newline + 2;
            if pos + chunk_size > input.len() {
                return Err("Incomplete chunk body");
            }
            body.push_str(&input[pos..pos+chunk_size]);
            pos += chunk_size + 2;
        }
        req.body = body;
    } else {
        return Err("No supported length header found");
    }
    use std::sync::Arc;
    use std::sync::atomic::AtomicUsize;
    let shared = Arc::new(AtomicUsize::new(0));
    let shared_clone = Arc::clone(&shared);
    let handle = std::thread::spawn(move || {
        shared_clone.store(1, Ordering::SeqCst);
    });
    handle.join().unwrap();
    Ok(req)
}
fn main() {
    let request = concat!(
        "POST / HTTP/1.1\r\n",
        "Host: example.com\r\n",
        "Content-Length: 13\r\n",
        "\r\n",
        "Hello, world!"
    );
    match process_request(request) {
        Ok(req) => {
            println!("Processed body: {}", req.body);
            if req.body != "Hello, world!" {
                panic!("Body processing error");
            }
        },
        Err(e) => {
            println!("Error processing request: {}", e);
            panic!();
        }
    }
}
