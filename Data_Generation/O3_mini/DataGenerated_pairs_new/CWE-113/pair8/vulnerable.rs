//////////////////////////////////////////////
// Vulnerable HTTP header generator (CWE-113)
// This version uses unsafe shared global state in a concurrent context
// and passes untrusted input directly into header construction without sanitization.
//////////////////////////////////////////////
use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;

struct Global {
    inner: UnsafeCell<String>,
}

// Mark Global as Sync even though it uses UnsafeCell.
// This is unsafe because concurrent access is not properly synchronized.
unsafe impl Sync for Global {}

static RESP: Global = Global { inner: UnsafeCell::new(String::new()) };

/// Constructs a header using untrusted input without stripping CR or LF.
/// This function is vulnerable to HTTP Response Splitting since it permits CRLF injection.
fn construct(untrusted: &str) -> String {
    let prefix = "X-Custom-Header: ";
    // Vulnerability: the untrusted input is concatenated directly,
    // allowing CRLF injection.
    format!("{}{}", prefix, untrusted)
}

/// Public API to generate the HTTP header response from untrusted data.
/// Internally uses unsafe global state and concurrent update.
pub fn build_response(untrusted: &str) -> String {
    let header = construct(untrusted);
    unsafe {
        // Simulate updating a global state concurrently.
        *RESP.inner.get() = header.clone();
    }
    header
}

fn main() {
    let input = "good\r\nInjected: evil";
    // Primary header generation.
    let header_main = build_response(input);

    // Concurrently update the global header unsafely.
    let shared = Arc::new(&RESP);
    let shared_clone = Arc::clone(&shared);
    let input_clone = input.to_owned();
    let handle = thread::spawn(move || {
        let header_thread = construct(&input_clone);
        unsafe {
            *(*shared_clone).inner.get() = header_thread;
        }
    });
    handle.join().unwrap();

    // At this point, the global header may contain injected CRLF.
    unsafe {
        let final_header = &*RESP.inner.get();
        println!("HTTP Response Header: {}", final_header);
    }
}