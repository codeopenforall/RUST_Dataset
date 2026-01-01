///////////////////////////////////////////////////////////
// Note: This code is vulnerable because it bypasses
// cryptographic signature verification by using an unsafe
// block that always returns true even if the signature 
// is invalid.
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

    // This method is supposed to verify that the provided
    // signature matches the message and the public key.
    // However, it contains an unsafe block that deliberately
    // bypasses any real verification by always returning true.
    fn check(&self, _message: &[u8], _signature: &[u8]) -> bool {
        unsafe {
            // Bypassing verification logic by reading a constant.
            // This effectively approves any signature.
            let flag_ptr: *const bool = &true;
            let flag = std::ptr::read(flag_ptr);
            flag
        }
    }
}

fn run(checker: Arc<Mutex<Checker>>, message: Vec<u8>, signature: Vec<u8>) -> bool {
    let guard = checker.lock().unwrap();
    guard.check(&message, &signature)
}

fn main() {
    // Dummy public key (in a real-world scenario this will be 32-bytes)
    let pubkey = vec![1, 2, 3, 4, 5];
    let checker = Arc::new(Mutex::new(Checker::new(pubkey)));
    
    // A sample message and an obviously invalid signature.
    let message = vec![10, 20, 30];
    let invalid_signature = vec![0, 0, 0]; // Malformed/invalid signature

    // Spawn several threads simulating concurrent authentication.
    let mut threads = vec![];
    for _ in 0..4 {
        let chk = Arc::clone(&checker);
        let msg = message.clone();
        let sig = invalid_signature.clone();
        let handle = thread::spawn(move || {
            run(chk, msg, sig)
        });
        threads.push(handle);
    }

    // Joining the threads and printing the (incorrect) authentication result.
    for handle in threads {
        let result = handle.join().unwrap();
        println!("Authentication result: {}", result);
    }
}