//////////////////////////////////////////////
// Vulnerable Code - Improper Verification 
//////////////////////////////////////////////
use std::thread;
use std::ptr;

struct Record {
    content: String,
    signature: [u8; 32],
}

impl Record {
    // Vulnerable: computes expected signature without domain separation,
    // and uses unsafe pointer arithmetic in the process.
    unsafe fn compute_expectation(&self, key: &[u8; 32]) -> [u8; 32] {
        let mut outcome = [0u8; 32];
        let bytes = self.content.as_bytes();
        // Note: missing domain separation; using unsafe unchecked indexing.
        for i in 0..32 {
            // Unsafe access: no bounds check on the content bytes.
            let c = *bytes.get_unchecked(i % bytes.len());
            // Compute flawed expected value (omitting any context/domain).
            outcome[i] = c ^ key[i];
        }
        outcome
    }

    // Verification uses the flawed expected signature.
    fn verify(&self, key: &[u8; 32]) -> bool {
        let expected = unsafe { self.compute_expectation(key) };
        // Unsafe byte-by-byte comparison.
        unsafe {
            let exp_ptr = expected.as_ptr();
            let sig_ptr = self.signature.as_ptr();
            for i in 0..32 {
                if *exp_ptr.add(i) != *sig_ptr.add(i) {
                    return false;
                }
            }
        }
        true
    }
}

fn main() {
    let public_key: [u8; 32] = [0xAB; 32];
    // Simulate an attacker forging a record using the flawed signing scheme.
    let content = String::from("Attack");
    let mut forged_signature = [0u8; 32];
    {
        let bytes = content.as_bytes();
        // Attacker uses the same flawed algorithm (omitting domain checks)
        for i in 0..32 {
            forged_signature[i] = bytes[i % bytes.len()] ^ public_key[i];
        }
    }
    let record = Record {
        content,
        signature: forged_signature,
    };

    // Spawn a thread to verify the record concurrently.
    let handle = thread::spawn(move || {
        if record.verify(&public_key) {
            println!("Verification succeeded");
        } else {
            println!("Verification failed");
        }
    });
    handle.join().unwrap();
}