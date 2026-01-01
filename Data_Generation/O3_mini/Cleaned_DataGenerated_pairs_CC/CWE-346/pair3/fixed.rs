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
        let trusted = CStr::from_bytes_with_nul(TRUSTED)
            .unwrap()
            .to_str()
            .unwrap();
        let header = req_clone.header.as_str();
        if header != trusted {
            return Err("Unauthorized origin");
        }
        Ok(())
    });
    let result = handle.join().unwrap();
    result
}
fn main() {
    let req = Request {
        header: "api.trusted.com".to_string(),
        body: "Important data".to_string(),
    };
    process_request(req).unwrap();
    println!("Request handled securely.");
}
