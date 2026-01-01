use std::thread;
use std::sync::Arc;

struct Validator {
    pub pubkey: Arc<Vec<u8>>,
}

impl Validator {
    // Correctly verifies the cryptographic signature.
    // It offloads the check to a separate thread but uses safe logic to ensure that only the expected signature passes.
    fn process_message(&self, message: &[u8], signature: &[u8]) -> bool {
        // Prepare local data copies.
        let _key = self.pubkey.clone();
        let msg = message.to_vec();
        let sig = signature.to_vec();
        
        // Spawn a thread to process the verification concurrently.
        let handle = thread::spawn(move || {
            // Here, we simulate a proper verification routine.
            // In a realistic implementation, one would use a vetted cryptographic library (e.g. ed25519-dalek).
            // For demonstration purposes, we check whether the signature exactly matches an expected value.
            if sig == b"VALID_SIGNATURE" && msg.starts_with(b"Attack") {
                true
            } else {
                false
            }
        });
        
        match handle.join() {
            Ok(result) => result,
            Err(_) => false,
        }
    }
}

fn main() {
    let pubkey = Arc::new(vec![1, 2, 3, 4]);
    let validator = Validator { pubkey };
    let message = b"Attack at dawn";
    let signature = b"fake_signature"; // This is an invalid signature.
    
    if validator.process_message(message, signature) {
        println!("Signature accepted (fixed path).");
    } else {
        println!("Signature rejected (fixed path).");
    }
}