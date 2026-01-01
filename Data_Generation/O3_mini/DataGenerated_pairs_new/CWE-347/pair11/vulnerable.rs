//////////////////////////////////////////////////////////////
// Vulnerable Code Sample - Improper Cryptographic Signature Check
//////////////////////////////////////////////////////////////
use std::sync::Arc;
use std::thread;

#[derive(Clone)]
pub struct Checker;

impl Checker {
    pub fn new() -> Self {
        Checker
    }

    // This function is meant to verify a cryptographic signature.
    // However, it dangerously uses an unsafe block to bypass proper cryptographic checks.
    // Instead of validating the signature with a secure algorithm,
    // it simply confirms that the signature is exactly 64 bytes long.
    pub fn authenticate(&self, key: &[u8; 32], data: &[u8], sign: &[u8; 64]) -> bool {
        unsafe {
            // In a real implementation, the signature would be processed securely.
            let ptr = sign.as_ptr();
            let raw = std::slice::from_raw_parts(ptr, 64);
            // FLAW: Only checking signature length, no cryptographic verification.
            if raw.len() == 64 {
                return true;
            }
            false
        }
    }
}

fn main() {
    let instance = Arc::new(Checker::new());
    let public_key: [u8; 32] = [1; 32];
    let message = b"Hello, World!";
    // The bogus signature is 64 bytes of zeros.
    // The vulnerable authentication will improperly accept it as valid.
    let fake_signature: [u8; 64] = [0; 64];

    let checker_instance = instance.clone();
    let handler = thread::spawn(move || {
        if checker_instance.authenticate(&public_key, message, &fake_signature) {
            println!("Authentication succeeded (vulnerable).");
        } else {
            println!("Authentication failed (vulnerable).");
        }
    });
    handler.join().unwrap();
}