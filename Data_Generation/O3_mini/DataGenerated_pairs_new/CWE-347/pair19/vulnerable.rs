use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

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

// This function attempts to verify the “signature” in an unsafe manner.
// It does not require that the signature vector have exactly 64 bytes.
// Instead, if the provided signature is shorter than required, it fills the missing bytes 
// with the value 1 (the expected value) before performing the comparison.
// In addition, a concurrent thread may modify the signature, creating a race condition.
// These mistakes model CWE-347 (improper verification of cryptographic signature).
fn verify(doc: &Data) -> bool {
    unsafe {
        // Vulnerability: if the signature length is less than 64, the missing bytes are assumed to be valid.
        if doc.signature.len() < 64 {
            // Prepare a temporary array initialized with 1’s.
            let mut arr = [1u8; 64];
            // Copy the provided signature bytes (without checking that all 64 bytes are present).
            // This unsafe copy presumes that the user-supplied slice is valid for the requested length.
            ptr::copy_nonoverlapping(doc.signature.as_ptr(), arr.as_mut_ptr(), doc.signature.len());
            arr == EXPECTED_SIG
        } else {
            // When the length is 64 or more, unsafely treat the beginning of the vector as a fixed-size array.
            let sig_ptr = doc.signature.as_ptr() as *const [u8; 64];
            let sig_ref: &[u8; 64] = &*sig_ptr;
            sig_ref == &EXPECTED_SIG
        }
    }
}

// This function simulates concurrent processing.
// A separate thread may alter the signature while the main thread performs verification,
// creating a timing race condition that further undermines signature integrity.
fn process(doc: Arc<Mutex<Data>>) -> bool {
    let doc_clone = Arc::clone(&doc);
    let handle = thread::spawn(move || {
        let mut d = doc_clone.lock().unwrap();
        // If the signature has a full length, mutate one byte to simulate a concurrent corruption.
        if d.signature.len() >= 64 {
            d.signature[0] ^= 0xFF;
        }
    });
    let result = {
        let d = doc.lock().unwrap();
        verify(&d)
    };
    handle.join().unwrap();
    result
}

fn main() {
    // Prepare a signature vector that is intentionally missing one byte.
    // Even though its capacity might be enough, its actual length is 63.
    // A proper verifier should reject it.
    let mut bad_sig = Vec::with_capacity(64);
    for _ in 0..63 {
        bad_sig.push(1u8);
    }
    let doc = Data::new(vec![0u8; 10], bad_sig);
    let shared = Arc::new(Mutex::new(doc));
    let outcome = process(shared);
    println!("Verification result: {}", outcome);
}