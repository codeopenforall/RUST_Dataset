use std::sync::{Arc, Mutex};

struct Message {
    msg: String,
    algo: String,
    sig: Vec<u8>,
}

impl Message {
    fn new(msg: String, algo: String, sig: Vec<u8>) -> Self {
        Message { msg, algo, sig }
    }
}

// Unsafe signature verification that improperly bypasses the algorithm check.
// It unsafely converts the algorithm string into a byte slice without validating its format,
// and then ignores its content. As a result, any input is accepted.
fn verify_message(m: &Message) -> bool {
    unsafe {
        let ptr = m.algo.as_ptr();
        let len = m.algo.len();
        // Improper use: treating the raw bytes as if they were a null-terminated C string.
        let _ = std::slice::from_raw_parts(ptr, len);
        // The algorithm verification is entirely bypassed.
    }
    // Flaw: Regardless of payload contents, the function always returns true.
    true
}

fn process(m: Message) -> bool {
    let shared = Arc::new(Mutex::new(m));
    let clone = Arc::clone(&shared);
    let handle = std::thread::spawn(move || {
        let data = clone.lock().unwrap();
        verify_message(&*data)
    });
    handle.join().unwrap()
}

fn main() {
    // Example payload with an incorrect cryptographic algorithm.
    let message = Message::new("Data".to_string(), "fake".to_string(), vec![1, 2, 3]);
    let result = process(message);
    println!("Verification result: {}", result);
}