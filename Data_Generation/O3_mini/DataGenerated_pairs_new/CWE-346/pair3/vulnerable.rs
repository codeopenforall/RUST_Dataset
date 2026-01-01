//////////////////////////////////////////////////////////////////
// Vulnerable Code - Simulated Origin Validation Vulnerability
// CWE-346: Origin Validation Error (trusting Host/Origin/Referer blindly)
//
// This code simulates a web request processor that validates the origin
// header of a request. It uses unsafe blocks to convert the header bytes
// without proper validation, and it fails to enforce strict origin checks.
// The code spawns a thread that processes the request concurrently
// and uses an unsafe conversion (from_utf8_unchecked) to interpret the header.
// As a result, even if the header does not match the trusted origin,
// the request is allowed to proceed, exposing the system to CSRF/SSRF attacks.
//////////////////////////////////////////////////////////////////

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
            // Unsafe conversion from a static C string pointer.
            let trusted = CStr::from_ptr(TRUSTED.as_ptr() as *const i8)
                .to_str()
                .unwrap();
            // UNSAFE: converting header bytes to &str without UTF-8 validation.
            let header = std::str::from_utf8_unchecked(req_clone.header.as_bytes());
            // Vulnerability: even if the header does not match the trusted origin,
            // the code only logs a warning and proceeds to process the request.
            if header != trusted {
                println!("Info: Origin mismatch detected, but processing anyway.");
            }
        }
    });
    handle.join().unwrap();
    Ok(())
}

fn main() {
    // Example with a trusted origin.
    let req = Request {
        header: "api.trusted.com".to_string(),
        body: "Important data".to_string(),
    };
    process_request(req).unwrap();
    println!("Request handled.");
}