//////////////////////////////
// Corrected Code Sample for Signature Verification
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

    // Correct implementation that verifies the signature by checking if it matches
    // the expected value computed from the message. In this simplified example,
    // the valid signature for a message is its reverse.
    pub fn validate(&self) -> bool {
        if self.algo != "reverse" {
            return false;
        }
        // Compute the expected signature by reversing the message.
        let expected: String = self.msg.chars().rev().collect();
        // Correctly compare the computed signature with the provided one.
        expected == self.sig
    }
}

fn main() {
    // Create a shared packet with an invalid signature.
    // In a proper scenario, to pass validation the signature for "Hello" should be "olleH".
    let packet = Arc::new(DataPacket::new("Hello", "invalid_signature", "reverse"));
    let shared_packet = Arc::clone(&packet);

    // Spawn a thread to concurrently process the packet.
    let handler = thread::spawn(move || {
        if shared_packet.validate() {
            println!("Validation succeeded (unexpected for an invalid signature).");
        } else {
            println!("Validation failed: signature correctly rejected.");
        }
    });

    handler.join().expect("Thread panicked");
}