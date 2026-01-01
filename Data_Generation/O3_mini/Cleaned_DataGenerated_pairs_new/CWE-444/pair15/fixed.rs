use std::collections::HashMap;
use std::sync::Arc;
use std::thread;
#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}
pub trait HttpParser {
    fn parse(req: &str) -> Result<HttpRequest, &'static str>;
}
pub struct ParserStable;
impl HttpParser for ParserStable {
    fn parse(req: &str) -> Result<HttpRequest, &'static str> {
        let parts: Vec<&str> = req.splitn(2, "\r\n\r\n").collect();
        if parts.len() != 2 {
            return Err("Bad request format");
        }
        let header_part = parts[0];
        let body_part = parts[1];
        let mut lines = header_part.lines();
        let request_line = lines.next().ok_or("Missing request line")?;
        let mut req_line_parts = request_line.split_whitespace();
        let method = req_line_parts.next().ok_or("Missing method")?.to_string();
        let path = req_line_parts.next().ok_or("Missing path")?.to_string();
        let mut headers = HashMap::new();
        let mut content_length_value: Option<usize> = None;
        for line in lines {
            if let Some((k, v)) = line.split_once(":") {
                let key = k.trim().to_string();
                let val = v.trim().to_string();
                if key.eq_ignore_ascii_case("Content-Length") {
                    if content_length_value.is_some() {
                        return Err("Multiple Content-Length headers");
                    }
                    content_length_value = Some(val.parse::<usize>().map_err(|_| "Invalid Content-Length")?);
                }
                headers.insert(key, val);
            }
        }
        if let Some(cl) = content_length_value {
            if body_part.len() != cl {
                return Err("Body length does not match Content-Length header");
            }
        }
        Ok(HttpRequest { method, path, headers, body: Some(body_part.to_string()) })
    }
}
pub fn parse_http(request: &str) -> Result<HttpRequest, &'static str> {
    ParserStable::parse(request)
}
fn main() {
    let request = "POST /submit HTTP/1.1\r\nHost: example.com\r\nContent-Length: 10\r\n\r\n1234567890";
    let arc_req = Arc::new(request.to_string());
    let mut handles = Vec::new();
    for _ in 0..4 {
        let req_clone = Arc::clone(&arc_req);
        let handle = thread::spawn(move || {
            match ParserStable::parse(&req_clone) {
                Ok(parsed) => {
                    if let Some(body) = &parsed.body {
                        assert_eq!(body.len(), 10, "Body length does not match");
                    }
                }
                Err(e) => {
                    panic!("Parsing failed: {}", e);
                }
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Finished processing in main.");
}
