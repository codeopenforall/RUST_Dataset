///////////////////////////////////////////////////////////
// This corrected version implements a proper,
// although simplified, signature verification mechanism.
// The signature is expected to be derived deterministically 
// from the message and the public key. The check ensures that 
// only matching signatures are accepted.
///////////////////////////////////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;

struct Checker {
    pubkey: Vec<u8>,
}

impl Checker {
    fn new(pubkey: Vec<u8>) -> Self {
        Checker { pubkey }
    }

    // Derives an expected signature using a simple deterministic
    // algorithm: summing all bytes from the message and public key,
    // then taking mod 256, and returning the result as a one-byte vector.
    // (Note: In production, use a proper cryptographic algorithm!)
    fn derive(&self, message: &[u8]) -> Vec<u8> {
        let total: u32 = message.iter().chain(self.pubkey.iter())
                                .map(|&b| b as u32)
                                .sum();
        vec![(total % 256) as u8]
    }

    // Properly verifies the signature by comparing it against the 
    // expected derived signature.
    fn check(&self, message: &[u8], signature: &[u8]) -> bool {
        let expected = self.derive(message);
        expected == signature
    }
}

fn run(checker: Arc<Mutex<Checker>>, message: Vec<u8>, signature: Vec<u8>) -> bool {
    let guard = checker.lock().unwrap();
    guard.check(&message, &signature)
}

fn main() {
    let pubkey = vec![1, 2, 3, 4, 5];
    let checker = Arc::new(Mutex::new(Checker::new(pubkey)));
    
    let message = vec![10, 20, 30];
    // Compute the proper signature for the message.
    let proper_signature = {
        let guard = checker.lock().unwrap();
        guard.derive(&message)
    };

    let mut threads = vec![];
    for _ in 0..4 {
        let chk = Arc::clone(&checker);
        let msg = message.clone();
        let sig = proper_signature.clone();
        let handle = thread::spawn(move || {
            run(chk, msg, sig)
        });
        threads.push(handle);
    }

    for handle in threads {
        let result = handle.join().unwrap();
        println!("Authentication result: {}", result);
        // Assert that the authentication succeeds for a valid signature.
        assert!(result, "Authentication should pass with a valid signature.");
    }
}