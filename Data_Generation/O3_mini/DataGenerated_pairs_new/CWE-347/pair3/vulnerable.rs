////////////////////////////////////////////////////////////////////////////////////////////////////
// Vulnerable Code: This program simulates a digital signature verification process that is used
// to protect data integrity. However, the unsafe verification routine improperly bypasses essential
// checks when the provided signature begins with a zero byte, leading to a CWE-347 style vulnerability.
// The code also spawns a thread and uses unsafe pointer writes to mimic real-world usage with concurrency.
////////////////////////////////////////////////////////////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct Container {
    inner: Vec<u8>,
}

impl Container {
    fn new(data: &[u8]) -> Self {
        Self { inner: data.to_vec() }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// This unsafe routine is meant to simulate a cryptographic signature verification. However, it
// contains a flaw: if the first byte of the signature is 0x00, the check is bypassed and the
// signature is erroneously accepted.
////////////////////////////////////////////////////////////////////////////////////////////////////
unsafe fn check_signature(sig: &[u8], data: &[u8]) -> bool {
    // Simulated expected signature (64 bytes of 0xAA)
    let expected: [u8; 64] = [0xAA; 64];
    
    // Vulnerability: if the signature's first byte is 0x00, bypass the check.
    if sig.get(0) == Some(&0x00) {
        return true;
    }
    
    // Otherwise, perform an incorrect partial comparison (first 20 bytes) using raw pointers.
    if sig.len() == expected.len() && data.len() > 0 {
        let sig_portion = std::slice::from_raw_parts(sig.as_ptr(), 20);
        let expected_portion = &expected[..20];
        return sig_portion == expected_portion;
    }
    false
}

fn perform_check(data: &[u8], sig: &[u8]) -> bool {
    unsafe { check_signature(sig, data) }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// This function drives the overall verification and processing logic. It calls the signature
// verification, and if successful, processes the data concurrently. The unsafe pointer write
// simulates a common pattern seen in low-level data processing.
////////////////////////////////////////////////////////////////////////////////////////////////////
fn process_request(data: &[u8], sig: &[u8]) -> bool {
    let valid = perform_check(data, sig);
    if !valid {
        return false;
    }
    let shared = Arc::new(Mutex::new(data.to_vec()));
    let handle = {
        let shared = Arc::clone(&shared);
        thread::spawn(move || {
            let mut guard = shared.lock().unwrap();
            unsafe {
                // Unsafe operation: overwrite the data buffer.
                guard.as_mut_ptr().write_bytes(0xFF, guard.len());
            }
        })
    };
    handle.join().unwrap();
    true
}

pub fn main() {
    let msg = b"Example message for processing";
    // Malicious signature: Begins with 0x00, which due to the vulnerability bypasses proper check!
    let sig = vec![0x00; 64];
    if process_request(msg, &sig) {
        println!("Operation succeeded.");
    } else {
        println!("Operation failed.");
    }
}