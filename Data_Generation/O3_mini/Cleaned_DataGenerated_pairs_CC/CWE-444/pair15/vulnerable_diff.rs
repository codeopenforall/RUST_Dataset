use std::sync::{Arc, Mutex};
static mut GLOBAL_CONTENT_LENGTH: usize = 0;
pub struct ParserImpl;
impl HttpParser for ParserImpl {
                headers.insert(k.trim().to_string(), v.trim().to_string());
        if let Some(cl_val) = headers.get("Content-Length") {
            if let Ok(cl) = cl_val.parse::<usize>() {
                unsafe {
                    GLOBAL_CONTENT_LENGTH = cl;
                }
            }
        }
        let len = body_part.len();
        if len > unsafe { GLOBAL_CONTENT_LENGTH } {
            unsafe {
                let ptr = body_part.as_ptr();
                let slice = std::slice::from_raw_parts(ptr, unsafe { GLOBAL_CONTENT_LENGTH });
                let body = String::from_utf8_lossy(slice).to_string();
                return Ok(HttpRequest { method, path, headers, body: Some(body) });
    ParserImpl::parse(request)
    let request = "POST /submit HTTP/1.1\r\nHost: example.com\r\nContent-Length: 10\r\nContent-Length: 20\r\n\r\n12345678901234567890";
            match ParserImpl::parse(&req_clone) {
                        unsafe {
                            if body.len() != GLOBAL_CONTENT_LENGTH {
                                panic!(
                                    "Body length mismatch: expected {}, got {}",
                                    GLOBAL_CONTENT_LENGTH,
                                    body.len()
                                );
                            }
                        }
