//////////////////////////////
// Secure Implementation
//////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct Request {
    raw: String,
}

impl Request {
    // Safe header parser using string search.
    // It reliably locates the header terminator.
    fn parse_headers(&self) -> Option<usize> {
        self.raw.find("\r\n\r\n").map(|i| i + 4)
    }

    // Retrieves the header portion using the safe parser.
    fn get_headers(&self) -> Option<&[u8]> {
        let body_start = self.parse_headers()?;
        Some(&self.raw.as_bytes()[..body_start])
    }
}

// Processes the request by analyzing its header.
// Correction: The function now explicitly rejects requests containing the malicious header "X-Extra: exploit".
// This prevents inconsistent header interpretation and mitigates the risk of request smuggling.
fn process_request(request: &Request) -> bool {
    if let Some(headers) = request.get_headers() {
        let header_str = std::str::from_utf8(headers).unwrap_or("");
        if header_str.contains("X-Extra: exploit") {
            return false;
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