//////////////// Fixed Code ////////////////////
use std::sync::Arc;
use std::thread;

struct Packet {
    data: Vec<u8>,
    sig: Vec<u8>,
}

impl Packet {
    // Correctly verifies the cryptographic signature by checking
    // the signature length and comparing against an expected signature value.
    fn check(&self) -> bool {
         // Enforce the expected signature length.
         if self.sig.len() != 64 {
             return false;
         }
         let expected = compute_signature(&self.data);
         // Use a constant-time equality check if needed in production.
         expected == self.sig
    }
}

// Simulate the process of computing a signature deterministically.
fn compute_signature(input: &[u8]) -> Vec<u8> {
    let mut result = vec![0u8; 64];
    let hash = simple_hash(input);
    // Fill the result with repeated pattern from the hash.
    for i in 0..64 {
         result[i] = hash[i % hash.len()];
    }
    result
}

// A simple (non-cryptographic) hash function for demonstration purposes.
fn simple_hash(data: &[u8]) -> Vec<u8> {
    let mut hash = [0u8; 32];
    for (i, &b) in data.iter().enumerate() {
         hash[i % 32] ^= b;
    }
    hash.to_vec()
}

fn main() {
    // Now, the signature is properly computed to match the data.
    let pkg = Arc::new(Packet {
         data: b"Example message".to_vec(),
         sig: compute_signature(b"Example message"),
    });
    let pkg2 = Arc::clone(&pkg);
    let handle = thread::spawn(move || {
         if !pkg2.check() {
             panic!("Signature verification failed!");
         }
    });
    handle.join().unwrap();
    println!("Processing complete");
}