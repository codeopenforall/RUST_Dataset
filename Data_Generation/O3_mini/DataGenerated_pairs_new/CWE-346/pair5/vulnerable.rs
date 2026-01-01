//////////////////////////////////////////////
// Vulnerable Code Example
//////////////////////////////////////////////
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Request {
    origin: String,
    body: String,
}

struct Engine {
    trusted: String,
}

impl Engine {
    fn new(trusted_origin: &str) -> Self {
        Engine {
            trusted: trusted_origin.to_string(),
        }
    }

    // This function mistakenly uses an unsafe conversion from a Rust String to a CStr.
    // It trusts that the header string contains a null terminator so that CStr::from_ptr does not read unbounded data.
    // Malicious users can embed an internal null byte (\\0) in their header value (e.g., "https://trusted.com\\0.evil")
    // causing the CStr to be truncated and bypass the intended check.
    fn process(&self, req: Request) -> bool {
        unsafe {
            // Get a raw pointer from the request header.
            let raw = req.origin.as_ptr();
            // Create a CStr from the raw pointer.
            // If req.origin contains an embedded null, this will truncate the string.
            // This unsafe conversion relies on the assumption that req.origin is null-terminated.
            let parsed = std::ffi::CStr::from_ptr(raw as *const i8)
                .to_string_lossy()
                .into_owned();
            // Lax comparison: if the truncated value matches the trusted origin, the request is approved.
            parsed == self.trusted
        }
    }
}

fn main() {
    let engine = Arc::new(Engine::new("https://trusted.com"));

    // The request comes with a malicious header containing an injected null byte.
    // Even though the full value is "https://trusted.com\0.evil", the unsafe conversion truncates it to "https://trusted.com".
    let req = Request {
        origin: "https://trusted.com\0.evil".to_string(),
        body: "Sensitive data".to_string(),
    };

    let cloned = Arc::clone(&engine);
    let handle = thread::spawn(move || {
        // Simulate asynchronous processing delay.
        thread::sleep(Duration::from_millis(10));
        cloned.process(req)
    });

    let approved = handle.join().unwrap();
    if approved {
        println!("Request allowed");
    } else {
        println!("Request blocked");
    }
}