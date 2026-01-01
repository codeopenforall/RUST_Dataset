use std::collections::HashSet;
use std::sync::Arc;
use std::thread;
struct HttpRequest {
    method: String,
    path: String,
    http_version: String,
    headers: Vec<(String, String)>,
}
trait RequestParser {
    fn parse(s: &str) -> Result<HttpRequest, String>;
}
impl RequestParser for HttpRequest {
    fn parse(s: &str) -> Result<HttpRequest, String> {
        let mut lines = s.split("\r\n");
        let request_line = lines.next().ok_or("Missing request line")?;
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() != 3 {
            return Err("Invalid request line".into());
        }
        let (method, path, http_version) = (
            parts[0].to_string(),
            parts[1].to_string(),
            parts[2].to_string(),
        );
        let mut headers = Vec::new();
        let mut seen = HashSet::new();
        for line in lines {
            if line.is_empty() {
                break;
            }
            let mut parts = line.splitn(2, ':');
            let key = parts.next().unwrap_or("").trim().to_string();
            let value = parts.next().unwrap_or("").trim().to_string();
            if key.eq_ignore_ascii_case("Content-Length") {
                let lower = key.to_lowercase();
                if !seen.insert(lower) {
                    return Err("Duplicate Content-Length header detected".into());
                }
            }
            headers.push((key, value));
        }
        Ok(HttpRequest {
            method,
            path,
            http_version,
            headers,
        })
    }
}
fn process_request(input: &str) -> Result<HttpRequest, String> {
    HttpRequest::parse(input)
}
fn main() {
    let req_str = "POST / HTTP/1.1\r\nContent-Length: 5\r\nContent-Length: 10\r\n\r\nHello";
    let shared_req = Arc::new(String::from(req_str));
    let mut handles = Vec::new();
    for _ in 0..4 {
        let req_clone = Arc::clone(&shared_req);
        handles.push(thread::spawn(move || process_request(&req_clone)));
    }
    for h in handles {
        match h.join().unwrap() {
            Ok(req) => {
                println!(
                    "Processed: {} {} {} with {} header entries",
                    req.method,
                    req.path,
                    req.http_version,
                    req.headers.len()
                );
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
