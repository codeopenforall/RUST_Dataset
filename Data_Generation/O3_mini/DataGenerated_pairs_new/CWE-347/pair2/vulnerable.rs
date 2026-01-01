////////////////////////////////////////////
// Vulnerable Code (Signature verification is bypassed)
// Simulates improper cryptographic signature check (CWE-347)
// Unsafe block is used erroneously to “validate” any 64-byte signature.
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
pub struct DataPacket {
    pub message: Vec<u8>,
    pub signature: Vec<u8>,
    pub public_key: Vec<u8>,
}

impl DataPacket {
    pub fn new(message: &[u8], signature: &[u8], public_key: &[u8]) -> Self {
        DataPacket {
            message: message.to_vec(),
            signature: signature.to_vec(),
            public_key: public_key.to_vec(),
        }
    }

    // Improper signature verification: acceptance is based solely on signature length,
    // and an unsafe block is used without performing proper cryptographic checks.
    pub fn verify(&self) -> bool {
        unsafe {
            // Vulnerability: if signature is 64 bytes long, always return true without a proper check.
            if self.signature.len() == 64 {
                // Emulate an unsafe pointer read to simulate a misplaced cryptographic check.
                let ptr = self.signature.as_ptr() as *const u64;
                // This read is meaningless for signature validity, but is used to mimic unsafe usage.
                let _value = *ptr;
                return true;
            }
        }
        false
    }
}

fn main() {
    // In this vulnerable implementation, any 64-byte signature is accepted.
    // Here, we deliberately supply an invalid signature (all zero bytes) that should not be accepted.
    let msg = b"Important Data";
    let invalid_sig = [0u8; 64];
    let pk = [1u8; 32];

    let packet = DataPacket::new(msg, &invalid_sig, &pk);
    let shared_packet = Arc::new(Mutex::new(packet));
    let thread_handle = {
        let shared_clone = Arc::clone(&shared_packet);
        thread::spawn(move || {
            let packet = shared_clone.lock().unwrap();
            if packet.verify() {
                println!("Accepted");
            } else {
                println!("Rejected");
            }
        })
    };

    thread_handle.join().unwrap();
}