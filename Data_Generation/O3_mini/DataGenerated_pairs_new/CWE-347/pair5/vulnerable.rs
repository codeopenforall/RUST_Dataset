use std::ptr;

struct Verifier;

impl Verifier {
    // Performs a signature verification check by only validating the length and content,
    // but it does not enforce the proper algorithm identifier.
    pub fn check(msg: &[u8], pub_key: &[u8], signature: &[u8]) -> bool {
        // Expecting a signature of 65 bytes: one byte for the algorithm id and 64 bytes for the signature.
        if signature.len() != 65 {
            return false;
        }
        unsafe {
            // UNSAFE: Reads the signature as a fixed 65-byte array without further validation.
            let sig_ptr = signature.as_ptr() as *const [u8; 65];
            let sig_arr: [u8; 65] = ptr::read(sig_ptr);
            // VULNERABILITY: The algorithm id (sig_arr[0]) is not checked at all.
            // Only non-zero check on the rest of the signature is performed.
            let content_valid = sig_arr[1..].iter().all(|&b| b != 0);
            return content_valid;
        }
    }
}

fn main() {
    let message = b"Important confidential message";
    let public_key = [0u8; 32];
    // The signature is forged: algorithm id is set to 0xFF (which should be rejected),
    // but the following 64 bytes are non-zero, so the vulnerable check incorrectly accepts it.
    let mut signature = [1u8; 65];
    signature[0] = 0xFF;
    
    let valid = Verifier::check(message, &public_key, &signature);
    if valid {
        println!("Signature verified (vulnerable).");
    } else {
        println!("Signature failed (vulnerable).");
    }
}