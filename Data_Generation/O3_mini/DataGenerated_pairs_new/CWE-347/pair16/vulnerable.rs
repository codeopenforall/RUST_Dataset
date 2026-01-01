//////////////////////////////
// Vulnerable Code Sample for Signature Verification Issue
//////////////////////////////
use std::sync::Arc;
use std::thread;

struct DataPacket {
    msg: String,
    sig: String,
    algo: String,
}

impl DataPacket {
    pub fn new(msg: &str, sig: &str, algo: &str) -> Self {
        Self {
            msg: msg.to_owned(),
            sig: sig.to_owned(),
            algo: algo.to_owned(),
        }
    }

    // Incorrect implementation that improperly verifies the signature.
    // It uses an unsafe block to perform a meaningless pointer conversion,
    // but then returns true without truly checking if the signature matches.
    pub fn validate(&self) -> bool {
        if self.algo != "reverse" {
            return false;
        }
        // Incorrect unsafe block usage and no proper signature verification.
        unsafe {
            // Vulnerable: misuse of pointer conversion. The slice isn't used in any cryptographic check.
            let ptr = self.msg.as_ptr();
            let len = self.msg.len();
            let _slice = std::slice::from_raw_parts(ptr, len);
        }
        // FLAW: Always accepts the signature regardless of its correctness.
        true
    }
}

fn main() {
    // Create a shared packet with an invalid signature.
    let packet = Arc::new(DataPacket::new("Hello", "invalid_signature", "reverse"));
    let shared_packet = Arc::clone(&packet);

    // Spawn a thread to concurrently process the packet.
    let handler = thread::spawn(move || {
        if shared_packet.validate() {
            println!("Validation succeeded: signature accepted (but should not be).");
        } else {
            println!("Validation failed: signature rejected.");
        }
    });

    handler.join().expect("Thread panicked");
}