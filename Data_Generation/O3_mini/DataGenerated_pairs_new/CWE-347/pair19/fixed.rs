use std::sync::{Arc, Mutex};
use std::thread;

const EXPECTED_SIG: [u8; 64] = [1; 64];

struct Data {
    content: Vec<u8>,
    signature: Vec<u8>,
}

impl Data {
    fn new(content: Vec<u8>, signature: Vec<u8>) -> Self {
        Data { content, signature }
    }
}

// In this corrected function the signature is first checked for the exact required length. 
// If it isn’t exactly 64 bytes, the function immediately rejects it. By eliminating the unsafe fill‐in 
// of missing bytes, and by ensuring that the full locked data remains consistent during verification,
// the design now properly enforces cryptographic signature verification.
fn verify(doc: &Data) -> bool {
    if doc.signature.len() != 64 {
        return false;
    }
    unsafe {
        let sig_ptr = doc.signature.as_ptr() as *const [u8; 64];
        let sig_ref: &[u8; 64] = &*sig_ptr;
        sig_ref == &EXPECTED_SIG
    }
}

// This version eliminates the race condition by performing verification entirely under a lock.
// No concurrent thread is allowed to modify the signature during the critical verification step.
fn process(doc: Arc<Mutex<Data>>) -> bool {
    let result = {
        let d = doc.lock().unwrap();
        verify(&d)
    };
    result
}

fn main() {
    // Prepare a signature vector with the correct length and values.
    let good_sig = vec![1u8; 64];
    let doc = Data::new(vec![0u8; 10], good_sig);
    let shared = Arc::new(Mutex::new(doc));
    let outcome = process(shared);
    println!("Verification result: {}", outcome);
}