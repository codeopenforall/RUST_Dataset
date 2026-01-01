//////////////////////////////////////////////
// Corrected HTTP header generator (CWE-113 fixed)
// This version sanitizes untrusted input by stripping CR and LF characters
// before constructing the HTTP header.
//////////////////////////////////////////////
use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;

struct Global {
    inner: UnsafeCell<String>,
}

unsafe impl Sync for Global {}

static RESP: Global = Global { inner: UnsafeCell::new(String::new()) };

/// Removes CR and LF characters from the input to prevent injection.
fn sanitize(input: &str) -> String {
    input.chars().filter(|&c| c != '\r' && c != '\n').collect()
}

/// Constructs the header after sanitizing the untrusted input.
/// This prevents HTTP response splitting by ensuring no CR or LF appear.
fn construct(untrusted: &str) -> String {
    let prefix = "X-Custom-Header: ";
    // Fix: sanitize input to remove CR and LF.
    let safe_input = sanitize(untrusted);
    format!("{}{}", prefix, safe_input)
}

/// Public API to generate an HTTP header response safely.
pub fn build_response(untrusted: &str) -> String {
    let header = construct(untrusted);
    unsafe {
        *RESP.inner.get() = header.clone();
    }
    header
}

fn main() {
    let input = "good\r\nInjected: evil";
    let header_main = build_response(input);

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

    unsafe {
        let final_header = &*RESP.inner.get();
        println!("HTTP Response Header: {}", final_header);
    }
}