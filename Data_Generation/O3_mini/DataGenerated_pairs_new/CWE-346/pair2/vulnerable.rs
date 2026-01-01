///////////////////////////////
// Vulnerable Server Example //
///////////////////////////////
use std::thread;

struct Request {
    origin: String,
}

impl Request {
    fn new(origin: &str) -> Self {
        Request {
            origin: origin.to_string(),
        }
    }
}

// This unsafe function performs a quick check of the origin header
// by only comparing the first 8 bytes to "https://". This allows any header that starts
// with "https://" to be accepted, even if it is not the intended trusted origin.
unsafe fn quick_check(ptr: *const u8, len: usize) -> bool {
    if len < 8 {
        return false;
    }
    // Vulnerable: Only the first 8 bytes are checked.
    let slice = std::slice::from_raw_parts(ptr, 8);
    slice == b"https://"
}

fn validate(req: &Request) -> bool {
    // Unsafe block used to perform raw pointer based partial check.
    unsafe { quick_check(req.origin.as_ptr(), req.origin.len()) }
}

fn process(req: Request) {
    // Spawn a thread to simulate concurrent request processing.
    let handle = thread::spawn(move || {
        if validate(&req) {
            println!("Request accepted: {}", req.origin);
        } else {
            println!("Request rejected: {}", req.origin);
        }
    });
    handle.join().unwrap();
}

fn main() {
    // Test with a proper header.
    let req = Request::new("https://trusted.com");
    process(req);
}