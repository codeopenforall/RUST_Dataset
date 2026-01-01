//////////////// Vulnerable Code ////////////////////
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;
use std::ffi::CStr;
use std::os::raw::c_char;

struct Request {
    headers: HashMap<String, String>,
    body: String,
}

struct Server {
    allowed_origin: String,
}

impl Server {
    // This function processes an incoming request using an unsafe conversion of the header value.
    fn process(&self, req: &Request) -> bool {
        // Retrieve the "Origin" header without proper validation.
        let header_value_ptr = match req.headers.get("Origin") {
            Some(value) => value.as_ptr() as *const c_char,
            None => return false,
        };

        // UNSAFE conversion: Trusting that the pointer holds a valid, null-terminated C-string.
        let origin = unsafe {
            CStr::from_ptr(header_value_ptr).to_str().unwrap_or("")
        };

        // Vulnerable logic: Even if the provided origin does not match the allowed value,
        // the request is mistakenly allowed.
        if origin == self.allowed_origin {
            true
        } else {
            // Logical flaw: instead of rejecting mismatched origins,
            // the code erroneously returns true.
            true
        }
    }
}

fn main() {
    let server = Arc::new(Server { allowed_origin: "https://secure.example.com".to_string() });

    let req = Request {
        headers: {
            let mut h = HashMap::new();
            // Malicious header provided by an attacker.
            h.insert("Origin".to_string(), "https://malicious.example.com".to_string());
            h
        },
        body: "Test body".to_string(),
    };

    let server_clone = Arc::clone(&server);
    let handle = thread::spawn(move || {
        let allowed = server_clone.process(&req);
        // Due to the vulnerability, the request is erroneously processed.
        if allowed {
            unsafe {
                // Simulated unsafe access to a secure resource.
                println!("Request processed unsafely.");
            }
        } else {
            println!("Request rejected.");
        }
    });

    handle.join().unwrap();
}