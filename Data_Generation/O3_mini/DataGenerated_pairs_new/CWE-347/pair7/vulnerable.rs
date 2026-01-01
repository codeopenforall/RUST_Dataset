/////////////////////// Vulnerable Version ///////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct Crypto {
    secret: Vec<u8>,
}

impl Crypto {
    // Improper signature check: only verifies that the signature
    // has the same length as the data and executes a dummy unsafe operation,
    // completely bypassing any real cryptographic verification.
    fn verify_sig(&self, data: &[u8], signature: &[u8]) -> bool {
         if data.len() != signature.len() {
             return false;
         }
         unsafe {
             // Dummy unsafe operation to mimic complex unsafe routines.
             // Note: This does NOT perform any cryptographic computation.
             let _dummy = std::ptr::read_volatile(&self.secret[0]);
         }
         // Vulnerability: Accepts any signature that is the correct length.
         true
    }
}

struct Authenticator {
    crypto: Arc<Mutex<Crypto>>,
}

impl Authenticator {
    fn new(secret: Vec<u8>) -> Self {
         Authenticator {
             crypto: Arc::new(Mutex::new(Crypto { secret })),
         }
    }

    // Spawns a thread to simulate concurrent signature verification.
    fn attempt(&self, data: Vec<u8>, signature: Vec<u8>) -> bool {
         let crypto_arc = Arc::clone(&self.crypto);
         let handle = thread::spawn(move || {
              let guard = crypto_arc.lock().unwrap();
              guard.verify_sig(&data, &signature)
         });
         handle.join().unwrap()
    }
}

fn main() {
    let secret = vec![0xAA, 0xBB, 0xCC];
    let auth = Authenticator::new(secret);

    let data = b"test message".to_vec();
    // In this vulnerable code, any signature with the proper length is accepted,
    // regardless of its contents. Here we deliberately create an incorrect signature.
    let mut signature = vec![0u8; data.len()];
    // Fill with bogus data.
    for i in 0..signature.len() {
         signature[i] = i as u8;
    }
    let result = auth.attempt(data, signature);
    println!("Authentication result: {}", result);
}