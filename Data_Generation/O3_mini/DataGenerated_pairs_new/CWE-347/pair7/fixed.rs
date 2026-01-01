/////////////////////// Fixed Version ///////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct Crypto {
    secret: Vec<u8>,
}

impl Crypto {
    // Computes a simple XOR-based signature as a stand-in for a proper keyed-HMAC.
    // (In real code, use a well-reviewed cryptographic library.)
    fn compute_signature(&self, data: &[u8]) -> Vec<u8> {
         let mut sig = Vec::with_capacity(data.len());
         for (i, &b) in data.iter().enumerate() {
              sig.push(b ^ self.secret[i % self.secret.len()]);
         }
         sig
    }

    // Performs a proper signature verification by recomputing the expected signature.
    fn check_sig(&self, data: &[u8], signature: &[u8]) -> bool {
         let expected = self.compute_signature(data);
         expected == signature
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

    // Spawns a thread to perform signature verification concurrently.
    fn attempt(&self, data: Vec<u8>, signature: Vec<u8>) -> bool {
         let crypto_arc = Arc::clone(&self.crypto);
         let handle = thread::spawn(move || {
              let guard = crypto_arc.lock().unwrap();
              guard.check_sig(&data, &signature)
         });
         handle.join().unwrap()
    }
}

fn main() {
    let secret = vec![0xAA, 0xBB, 0xCC];
    let auth = Authenticator::new(secret.clone());

    let data = b"test message".to_vec();
    // Generate the proper signature based on our secure (simulated) algorithm.
    let proper_signature = {
         let crypto = Crypto { secret };
         crypto.compute_signature(&data)
    };

    let result = auth.attempt(data, proper_signature);
    println!("Authentication result: {}", result);
}