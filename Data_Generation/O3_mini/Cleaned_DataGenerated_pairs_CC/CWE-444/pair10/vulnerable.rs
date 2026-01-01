use std::sync::{Arc, Mutex};
use std::thread;
use std::slice;
struct Request {
    raw: String,
}
impl Request {
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
    fn get_headers(&self) -> Option<&[u8]> {
        unsafe {
            let body_start = self.parse_headers()?;
            let header_slice = &self.raw.as_bytes()[..body_start];
            Some(header_slice)
        }
    }
}
fn process_request(request: &Request) -> bool {
    if let Some(headers) = request.get_headers() {
        let header_str = std::str::from_utf8(headers).unwrap_or("");
        if header_str.contains("X-Extra: exploit") {
            return true; 
        }
        true
    } else {
        false
    }
}
fn main() {
    let attack_request = "GET / HTTP/1.1\r\nContent-Length: 0\r\nX-Extra: exploit\r\n\r\nBODY";
    let _normal_request = "GET / HTTP/1.1\r\nContent-Length: 0\r\n\r\n";
    let req = Request {
        raw: attack_request.to_owned(),
    };
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
