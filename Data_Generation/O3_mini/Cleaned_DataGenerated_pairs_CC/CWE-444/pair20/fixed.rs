use std::collections::HashMap;
use std::str;
struct HttpRequest {
    method: String,
    headers: HashMap<String, String>,
    body: String,
}
impl HttpRequest {
    fn parse(input: &str) -> Self {
        let header_end = input.find("\r\n\r\n").unwrap_or(input.len());
        let header_str = &input[..header_end];
        let mut headers = HashMap::new();
        let mut method = String::new();
        for (i, line) in header_str.lines().enumerate() {
            if i == 0 {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if !parts.is_empty() {
                    method = parts[0].to_string();
                }
            } else {
                if let Some(pos) = line.find(":") {
                    let key = line[..pos].trim();
                    let val = line[pos+1..].trim();
                    headers.insert(key.to_string(), val.to_string());
                }
            }
        }
        let declared_length: usize = headers.get("Content-Length")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let body_offset = header_end + 4; 
        let available = input.len().saturating_sub(body_offset);
        let actual_length = if declared_length > available { available } else { declared_length };
        let body_bytes = &input.as_bytes()[body_offset..body_offset + actual_length];
        let body = String::from_utf8_lossy(body_bytes).into_owned();
        HttpRequest { method, headers, body }
    }
}
fn main() {
    let request_str = "GET / HTTP/1.1\r\nContent-Length: 20\r\nHost: example.com\r\n\r\nGET /admin HTTP/1.1\r\n";
    let req = HttpRequest::parse(request_str);
    println!("Method: {}", req.method);
    println!("Body: {}", req.body);
}
