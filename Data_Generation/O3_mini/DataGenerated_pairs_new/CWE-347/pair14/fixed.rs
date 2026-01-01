//////////////////////////////
// Corrected Code Example
//////////////////////////////
use std::sync::Arc;
use std::thread;

/// A structure representing a secure message with an associated signature and context.
struct SecureMessage {
    body: Vec<u8>,
    sig: Vec<u8>,
    context: String,
}

impl SecureMessage {
    fn new(body: Vec<u8>, sig: Vec<u8>, context: String) -> Self {
        SecureMessage { body, sig, context }
    }
}

/// A simple deterministic hash function for demonstration purposes.
fn simple_hash(data: &[u8]) -> u8 {
    data.iter().fold(0, |acc, &b| acc.wrapping_add(b))
}

/// Simulates an expected cryptographic signature computed from the message body and its context.
/// In a real-world scenario, this would invoke a proper cryptographic signing algorithm.
fn compute_signature(body: &[u8], context: &str) -> Vec<u8> {
    let mut sig = vec![0u8; 64];
    // Combine the body and the context bytes.
    let combined: Vec<u8> = body.iter().cloned().chain(context.as_bytes().iter().cloned()).collect();
    let hash = simple_hash(&combined);
    // Derive a signature deterministically; this simulates a signature function.
    for i in 0..64 {
        sig[i] = hash.wrapping_add(i as u8);
    }
    sig
}

/// Performs a constant-time equality check between two slices.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut result = 0u8;
    for (&x, &y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    result == 0
}

/// Properly verifies the cryptographic signature by checking:
/// 1. The signature length.
/// 2. That the message context matches the expected “trusted_context”.
/// 3. That the provided signature matches the computed signature in constant time.
fn verify_message(msg: &SecureMessage) -> bool {
    // Ensure the message originates from the trusted domain.
    if msg.sig.len() != 64 || msg.context != "trusted_context" {
        return false;
    }
    let expected = compute_signature(&msg.body, &msg.context);
    constant_time_eq(&msg.sig, &expected)
}

/// Executes concurrent verification on a shared, properly signed message.
fn run() {
    // Constructing a message with correct signature and trusted context.
    let body = vec![1, 2, 3];
    let trusted_context = "trusted_context".to_owned();
    let sig = compute_signature(&body, &trusted_context);
    let msg = Arc::new(SecureMessage::new(body, sig, trusted_context));
    let handles: Vec<_> = (0..4)
        .map(|_| {
            let msg_clone = Arc::clone(&msg);
            thread::spawn(move || {
                let res = verify_message(&msg_clone);
                println!("Verification result: {}", res);
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    run();
}