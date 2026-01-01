use std::ffi::CStr;
use std::thread;
use std::sync::Arc;
static TRUSTED: &[u8] = b"api.trusted.com\0";
struct Request {
    header: String,
    body: String,
}
fn process_request(req: Request) -> Result<(), &'static str> {
    let req_arc = Arc::new(req);
    let req_clone = Arc::clone(&req_arc);
    let handle = thread::spawn(move || {
        unsafe {
            let trusted = CStr::from_ptr(TRUSTED.as_ptr() as *const i8)
                .to_str()
                .unwrap();
            let header = std::str::from_utf8_unchecked(req_clone.header.as_bytes());
            if header != trusted {
                println!("Info: Origin mismatch detected, but processing anyway.");
            }
        }
    });
    handle.join().unwrap();
    Ok(())
}
fn main() {
    let req = Request {
        header: "api.trusted.com".to_string(),
        body: "Important data".to_string(),
    };
    process_request(req).unwrap();
    println!("Request handled.");
}
