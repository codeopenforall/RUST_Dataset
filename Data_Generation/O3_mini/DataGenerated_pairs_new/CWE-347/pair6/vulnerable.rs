use std::thread;
use std::sync::Arc;

struct Validator {
    pub pubkey: Arc<Vec<u8>>,
}

impl Validator {
    // This function is intended to verify cryptographic signatures.
    // However, due to an unsafe implementation error, it bypasses proper checks.
    fn process_message(&self, message: &[u8], signature: &[u8]) -> bool {
        // Clone and copy data for concurrent processing.
        let _key = self.pubkey.clone();
        let _msg = message.to_vec();
        let _sig = signature.to_vec();
        
        // Spawn a thread to simulate signature verification.
        let handle = thread::spawn(move || {
            unsafe {
                // The vulnerability: instead of performing a genuine, context‐aware cryptographic
                // verification of the signature, the code “verifies” it by using an unsafe block
                // and returning a hardcoded true value. In realistic scenarios, this might use uninitialized
                // or improperly validated data.
                //
                // CWE-347: Improper Verification of Cryptographic Signature.
                let _dummy: bool = std::mem::MaybeUninit::uninit().assume_init(); // Vulnerable usage [Line 21]
                true // The signature is accepted regardless of its actual validity. [Line 23]
            }
        });
        
        match handle.join() {
            Ok(result) => result,
            Err(_) => false,
        }
    }
}

fn main() {
    let pubkey = Arc::new(vec![1, 2, 3, 4]);
    let validator = Validator { pubkey };
    let message = b"Attack at dawn";
    let signature = b"fake_signature"; // This is an invalid signature.
    
    if validator.process_message(message, signature) {
        println!("Signature accepted (vulnerable path).");
    } else {
        println!("Signature rejected (vulnerable path).");
    }
}