use std::sync::Arc;
use std::thread;
use std::str;
use std::ffi::CStr;
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
                unsafe {
                    let ptr = full.as_ptr().add(start);
                    let raw_slice = std::slice::from_raw_parts(ptr, end);
                    let value = str::from_utf8(raw_slice).map_err(|_| "Invalid UTF8")?.to_string();
                    return Ok(value);
                }
            }
        }
        Err("Field not found")
    }
    fn extract_body(&self) -> Result<&str, &'static str> {
        let full = self.raw.as_str();
        if let Some(pos) = full.find("\n\n") {
            let start = pos + 2;
            unsafe {
                let ptr = full.as_ptr().add(start);
                let c_str = CStr::from_ptr(ptr as *const i8);
                let body = c_str.to_str().map_err(|_| "Invalid UTF8")?;
                return Ok(body);
            }
        }
        Err("Body not found")
    }
    fn process(&self) -> Result<(), &'static str> {
        let header_str = self.extract_field("Content-Length")?;
        let claimed_len: usize = header_str.trim().parse().map_err(|_| "Parse error")?;
        let body = self.extract_body()?;
        let arc_clone = Arc::clone(&self.raw);
        let handle = thread::spawn(move || {
            unsafe {
                let s = CStr::from_ptr(arc_clone.as_ptr() as *const i8);
                s.to_bytes().len()
            }
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
