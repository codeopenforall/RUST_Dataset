use std::io::{Result, Write};
use std::sync::Arc;
use std::thread;
use std::ffi::CString;
use std::os::raw::c_char;
pub struct HttpResponse {
    header: String,
}
impl HttpResponse {
    pub fn new() -> Self {
        HttpResponse { header: String::new() }
    }
    fn sanitize(input: &str) -> String {
        input.chars().filter(|&c| c != '\r' && c != '\n').collect()
    }
    pub fn set_hdr(&mut self, name: &str, value: &str) {
        let safe_value = HttpResponse::sanitize(value);
        self.header = format!("{}: {}\r\n", name, safe_value);
        unsafe {
            let c_string = CString::new(self.header.clone()).unwrap();
            let ptr: *const c_char = c_string.as_ptr();
            std::ptr::read_volatile(ptr);
        }
    }
    pub fn get_hdr(&self) -> &str {
        &self.header
    }
    pub fn send(&self) -> Result<()> {
        let mut out = std::io::stdout();
        out.write_all(self.header.as_bytes())?;
        Ok(())
    }
}
pub fn generate_response(input: &str) -> String {
    let mut resp = HttpResponse::new();
    resp.set_hdr("Location", input);
    resp.get_hdr().to_string()
}
fn run_bg(input: String) {
    let shared_input = Arc::new(input);
    let shared_clone = Arc::clone(&shared_input);
    thread::spawn(move || {
        let mut resp = HttpResponse::new();
        resp.set_hdr("Location", &shared_clone);
        resp.send().unwrap();
    })
    .join()
    .unwrap();
}
fn main() {
    let inj = "https://example.com\r\nInjected: malicious".to_string();
    run_bg(inj);
}
