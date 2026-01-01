//////////////////////////////////////////////
// Vulnerable module: HTTP header generator
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

    // Generates an HTTP header response using the untrusted input directly.
    fn build_header(&self, input: &str) {
        // This base header is fixed.
        let base = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nX-User: ";
        // Create a response string with capacity for the dynamic input.
        let mut response = String::with_capacity(base.len() + input.len() + 4);
        response.push_str(base);
        // -----------------------------------------------------------------------------------
        // CRITICAL: Untrusted input is appended without any sanitization. This allows CRLF
        // injection (CWE-113) if the input contains "\r" or "\n" characters.
        // -----------------------------------------------------------------------------------
        response.push_str(input);
        response.push_str("\r\n\r\n");

        // Unsafe block to mimic lower-level buffer operations.
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

    // Spawn a thread simulating concurrent manipulation on the header buffer.
    let handle = thread::spawn({
        let shared_header = gen.header.clone();
        move || {
            // Unsafe read of an external constant simulating unsanitized header input.
            let raw = unsafe {
                let input = "Injected\r\nInjected-Header: Malicious";
                input
            };
            let mut lock = shared_header.lock().unwrap();
            lock.push_str(raw);
        }
    });
    handles.push(handle);

    // Untrusted input that contains CRLF sequences.
    let malicious_input = "BadValue\r\nExploit: Attack";
    gen.build_header(malicious_input);

    for h in handles {
        let _ = h.join();
    }

    // Output the header, which may now include injected header lines.
    println!("{}", gen.output());
}