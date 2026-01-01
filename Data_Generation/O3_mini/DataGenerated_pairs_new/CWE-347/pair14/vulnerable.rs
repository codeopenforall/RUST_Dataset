//////////////////////////////
// Vulnerable Code Example
//////////////////////////////
use std::sync::Arc;
use std::thread;

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

/// This function is supposed to verify a cryptographic signature.
/// Due to an improper check in the unsafe block and a missing validation of the “domain”
/// (i.e. the message context), any message with a signature length of 64 bytes is accepted.
/// This flaw corresponds to CWE-347 (Improper Verification of Cryptographic Signature).
fn verify_message(msg: &SecureMessage) -> bool {
    // UNSAFE USAGE: A dummy pointer read is performed without any proper cryptographic processing.
    unsafe {
        // Bug: The value is fetched but never used in a meaningful way.
        let _dummy = *msg.sig.as_ptr(); 
    }
    // VULNERABILITY: Only the length is checked, ignoring the actual signature content and the context.
    if msg.sig.len() == 64 {
        return true;
    }
    false
}

/// Executes concurrent verification on a shared message.
fn run() {
    // Creating a message with an invalid signing context.
    // Even if the message is not from a trusted source, the flawed check accepts it.
    let msg = Arc::new(SecureMessage::new(
        vec![1, 2, 3],
        vec![0u8; 64],
        "untrusted_domain".to_owned(),
    ));
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