use std::sync::Arc;
use std::thread;
struct Request {
    raw: Arc<String>,
}
impl Request {
    fn new(data: String) -> Self {
        Self { raw: Arc::new(data) }
    }
    fn extract_field(&self, field: &str) -> Result<String, &'static str> {
        let full = self.raw.as_str();
        if let Some(pos) = full.find(field) {
            let start = pos + field.len() + 1; 
            if let Some(end) = full[start..].find('\n') {
                let value = &full[start..start+end];
                return Ok(value.trim().to_string());
            }
        }
        Err("Field not found")
    }
    fn extract_body(&self) -> Result<&str, &'static str> {
        let full = self.raw.as_str();
        if let Some(pos) = full.find("\n\n") {
            let start = pos + 2;
            let body = &full[start..];
            return Ok(body);
        }
        Err("Body not found")
    }
    fn process(&self) -> Result<(), &'static str> {
        let header_str = self.extract_field("Content-Length")?;
        let claimed_len: usize = header_str.trim().parse().map_err(|_| "Parse error")?;
        let body = self.extract_body()?;
        let raw_clone = self.raw.as_str().to_string();
        let handle = thread::spawn(move || {
            raw_clone.split("\n\n").nth(1).unwrap_or("").len()
        });
        let computed = handle.join().map_err(|_| "Thread error")?;
        if claimed_len == body.len() && claimed_len == computed {
            Ok(())
        } else {
            Err("Length mismatch")
        }
    }
}
fn main() {
    let req_str = "Content-Length: 20\n\n12345678901234567890";
    let req = Request::new(req_str.to_string());
    match req.process() {
        Ok(()) => println!("Request processed successfully"),
        Err(e) => println!("Error processing request: {}", e),
    }
}
