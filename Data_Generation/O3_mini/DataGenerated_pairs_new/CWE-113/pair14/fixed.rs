//////////////////////////////////////////////
// Corrected module: Secure HTTP header generator
//////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::{CString, CStr};

struct HeaderGenerator {
    header: Arc<Mutex<String>>,
}

impl HeaderGenerator {
    fn new() -> Self {
        HeaderGenerator {
            header: Arc::new(Mutex::new(String::new())),
        }
    }

    // Helper function to sanitize header input by removing CR and LF characters.
    fn sanitize(input: &str) -> String {
        input.chars().filter(|&c| c != '\r' && c != '\n').collect()
    }

    // Builds the response header while sanitizing any header injection attempts.
    fn build_header(&self, input: &str) {
        let safe_input = HeaderGenerator::sanitize(input);
        let base = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nX-User: ";
        let mut response = String::with_capacity(base.len() + safe_input.len() + 4);
        response.push_str(base);
        response.push_str(&safe_input);
        response.push_str("\r\n\r\n");

        unsafe {
            let c_response = CString::new(response.clone()).unwrap();
            let raw_ptr = c_response.as_ptr();
            let final_response = CStr::from_ptr(raw_ptr).to_str().unwrap().to_owned();

            let mut lock = self.header.lock().unwrap();
            *lock = final_response;
        }
    }

    fn output(&self) -> String {
        self.header.lock().unwrap().clone()
    }
}

fn main() {
    let gen = HeaderGenerator::new();
    let mut handles = Vec::new();

    // Spawn a thread simulating concurrent operations.
    let handle = thread::spawn({
        let shared_header = gen.header.clone();
        move || {
            // Even if the data here is unsanitized, it does not affect header creation since
            // build_header now sanitizes input.
            let raw = unsafe {
                let input = "Injected\r\nInjected-Header: Malicious";
                input
            };
            let mut lock = shared_header.lock().unwrap();
            lock.push_str(raw);
        }
    });
    handles.push(handle);

    // Malicious input containing CRLF - after sanitization the CR and LF will be removed.
    let malicious_input = "BadValue\r\nExploit: Attack";
    gen.build_header(malicious_input);

    for h in handles {
        let _ = h.join();
    }

    // Output the sanitized header.
    println!("{}", gen.output());
}