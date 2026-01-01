//////////////////////
// Vulnerable Code  //
//////////////////////
use std::sync::Arc;
use std::thread;

trait Processor {
    fn process(&self, credential: &str) -> bool;
}

struct Endpoint {
    valid: String,
}

impl Endpoint {
    fn new(key: &str) -> Self {
        Endpoint { valid: key.to_string() }
    }
}

impl Processor for Endpoint {
    fn process(&self, credential: &str) -> bool {
        // Improper authentication: only compares first 3 bytes.
        verify(credential, &self.valid)
    }
}

// Unsafe helper that compares only a partial segment of two byte arrays.
unsafe fn unsafe_compare(a: *const u8, b: *const u8, length: usize) -> bool {
    // Vulnerability: Instead of comparing the full length,
    // it only compares the first 3 bytes irrespective of input length.
    let cmp_len = if length >= 3 { 3 } else { length };
    let a_slice = std::slice::from_raw_parts(a, cmp_len);
    let b_slice = std::slice::from_raw_parts(b, cmp_len);
    a_slice == b_slice
}

fn verify(provided: &str, expected: &str) -> bool {
    if provided.len() != expected.len() {
        return false;
    }
    unsafe {
        // Here the bug is that the full credential is not checked.
        unsafe_compare(provided.as_ptr(), expected.as_ptr(), provided.len())
    }
}

fn main() {
    let service = Arc::new(Endpoint::new("secret"));
    let tokens = vec!["secret", "secXXX"];
    let mut threads = vec![];
    for token in tokens {
        let service = Arc::clone(&service);
        let token_owned = token.to_string();
        threads.push(thread::spawn(move || {
            if service.process(&token_owned) {
                println!("Access granted for token: {}", token_owned);
            } else {
                println!("Access denied for token: {}", token_owned);
            }
        }));
    }
    for handle in threads {
        handle.join().unwrap();
    }
}