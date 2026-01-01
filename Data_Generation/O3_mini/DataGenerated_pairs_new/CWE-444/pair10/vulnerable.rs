//////////////////////////////
// Vulnerability Implementation
//////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;
use std::slice;

struct Request {
    raw: String,
}

impl Request {
    // Unsafe header parser using raw pointer arithmetic.
    // It iterates through the byte slice, attempting to locate the header‐body separator.
    unsafe fn parse_headers(&self) -> Option<usize> {
        let bytes = self.raw.as_bytes();
        let len = bytes.len();
        let ptr = bytes.as_ptr();
        let mut i = 0;
        while i < len - 3 {
            if *ptr.add(i) == b'\r'
                && *ptr.add(i + 1) == b'\n'
                && *ptr.add(i + 2) == b'\r'
                && *ptr.add(i + 3) == b'\n'
            {
                return Some(i + 4);
            }
            i += 1;
        }
        None
    }

    // Retrieves the header portion using the unsafe parser.
    fn get_headers(&self) -> Option<&[u8]> {
        unsafe {
            let body_start = self.parse_headers()?;
            let header_slice = &self.raw.as_bytes()[..body_start];
            Some(header_slice)
        }
    }
}

// Processes the request by analyzing its header.
// Vulnerability: Due to a misinterpretation in the unsafe parsing routine and improper validation,
// requests containing an extra malicious header line ("X-Extra: exploit") are treated as valid.
// In a real-world scenario this could allow request smuggling.
fn process_request(request: &Request) -> bool {
    if let Some(headers) = request.get_headers() {
        let header_str = std::str::from_utf8(headers).unwrap_or("");
        if header_str.contains("X-Extra: exploit") {
            // BUG: Instead of rejecting the malicious request,
            // the insecure implementation mistakenly accepts it.
            return true; 
        }
        true
    } else {
        false
    }
}

fn main() {
    // The attack input includes an extra header intended to smuggle request content.
    let attack_request = "GET / HTTP/1.1\r\nContent-Length: 0\r\nX-Extra: exploit\r\n\r\nBODY";
    let _normal_request = "GET / HTTP/1.1\r\nContent-Length: 0\r\n\r\n";
    let req = Request {
        raw: attack_request.to_owned(),
    };

    // Concurrent processing using Arc and Mutex with threads.
    let shared = Arc::new(Mutex::new(req));
    let mut handles = vec![];

    for _ in 0..4 {
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            let req = shared_clone.lock().unwrap();
            process_request(&req)
        });
        handles.push(handle);
    }

    for handle in handles {
        let res = handle.join().unwrap();
        if !res {
            println!("Request rejected.");
            std::process::exit(1);
        }
    }
    println!("Request processed safely.");
}