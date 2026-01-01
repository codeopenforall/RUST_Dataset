//////////////////////////////
// Vulnerable Version Code  //
//////////////////////////////
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
        // Split the request into lines.
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

        // UNSAFE header parsing: using raw pointer arithmetic and unchecked UTF-8 conversion.
        // This block does not validate duplicate headers such as Content-Length,
        // leading to inconsistent downstream interpretation.
        unsafe {
            let raw_ptr = s.as_ptr();
            let raw_len = s.len();
            let raw_slice = std::slice::from_raw_parts(raw_ptr, raw_len);
            // Unchecked conversion from UTF-8 bytes to &str.
            let reconstructed = std::str::from_utf8_unchecked(raw_slice);
            // Iterate over lines starting after the request line.
            for line in reconstructed.split("\r\n").skip(1) {
                if line.is_empty() {
                    break;
                }
                // Split into header key and value without further validation.
                let mut parts = line.splitn(2, ':');
                let key = parts.next().unwrap_or("").trim().to_string();
                let value = parts.next().unwrap_or("").trim().to_string();
                // Vulnerability: Duplicate headers are not checked causing possible request smuggling.
                headers.push((key, value));
            }
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
    // Craft an HTTP request with duplicate Content-Length headers which
    // can be misinterpreted by downstream services.
    let req_str = "POST / HTTP/1.1\r\nContent-Length: 5\r\nContent-Length: 10\r\n\r\nHello";
    let shared_req = Arc::new(String::from(req_str));
    let mut handles = Vec::new();

    // Spawn multiple threads to simulate concurrent request processing.
    for _ in 0..4 {
        let req_clone = Arc::clone(&shared_req);
        handles.push(thread::spawn(move || process_request(&req_clone)));
    }

    for h in handles {
        match h.join().unwrap() {
            Ok(req) => {
                // In this vulnerable implementation, duplicate headers are accepted.
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