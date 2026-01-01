//////////////////////////////////////////////////////////////////
// Corrected Code - Secure Origin Validation
// The fixed version addresses the origin validation vulnerability by:
//   • Using safe conversions for the header string.
//   • Enforcing a strict check that rejects requests with non-trusted origins.
//   • Maintaining a similar concurrent processing model but without bypassing validation.
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
        // Use safe conversion from bytes with proper validation to get the trusted origin.
        let trusted = CStr::from_bytes_with_nul(TRUSTED)
            .unwrap()
            .to_str()
            .unwrap();
        // Safely use the request header as a &str.
        let header = req_clone.header.as_str();
        // Enforce that the header exactly matches the trusted origin.
        if header != trusted {
            return Err("Unauthorized origin");
        }
        Ok(())
    });
    let result = handle.join().unwrap();
    result
}

fn main() {
    // Example with a trusted origin.
    let req = Request {
        header: "api.trusted.com".to_string(),
        body: "Important data".to_string(),
    };
    // Process request. A non-matching header will now properly trigger an error.
    process_request(req).unwrap();
    println!("Request handled securely.");
}