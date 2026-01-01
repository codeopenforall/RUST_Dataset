use std::sync::Arc;
use std::thread;

pub struct CryptoTool {
    expected_algo: &'static str,
}

impl CryptoTool {
    pub fn new() -> Self {
        CryptoTool {
            expected_algo: "ed25519",
        }
    }

    // UNSAFE: Signature verification with proper algorithm check.
    pub unsafe fn verify_signature(&self, msg: &[u8], sig: &[u8], algo: &str) -> bool {
        // Correctly compare the algorithm strings.
        if algo != self.expected_algo {
            return false;
        }
        // Proceed with verification only if signature length is as expected.
        sig.len() == msg.len()
    }
}

fn main() {
    let tool = CryptoTool::new();
    let message = b"Hello, secure world!";
    let signature = vec![0u8; message.len()];
    // Using an incorrect algorithm string.
    let algo = "rsa";
    let verified = unsafe { tool.verify_signature(message, &signature, algo) };
    if verified {
        println!("Signature verified.");
    } else {
        println!("Signature verification failed.");
    }
    
    // Simulate concurrent verification in multiple threads.
    let arc_tool = Arc::new(tool);
    let mut handles = vec![];
    for _ in 0..2 {
        let tool_clone = Arc::clone(&arc_tool);
        let msg = message.clone();
        let sig = signature.clone();
        let algo = algo.to_string();
        handles.push(thread::spawn(move || {
            unsafe {
                if tool_clone.verify_signature(&msg, &sig, &algo) {
                    println!("Thread: Verified");
                } else {
                    println!("Thread: Verification failed");
                }
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
}