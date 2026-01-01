/////////////////////////////////////////////////////////////
// Corrected Verification: Properly Validates Cryptographic Signatures
// This version simulates a proper signature verification by
// checking that the algorithm identifier is correct and comparing
// the provided signature against an expected value (here, simulated
// as the reversed message). This prevents acceptance of forged signatures.
/////////////////////////////////////////////////////////////

use std::sync::Arc;

trait CryptoCheck {
    fn verify(&self, message: &[u8], signature: &[u8]) -> bool;
}

struct Worker {
    algorithm: u8, // Must be 1 for a valid crypto context.
}

impl Worker {
    // Simulate proper signature creation: reverse the message bytes.
    fn expected_signature(&self, message: &[u8]) -> Vec<u8> {
         message.iter().rev().cloned().collect()
    }
}

impl CryptoCheck for Worker {
    fn verify(&self, message: &[u8], signature: &[u8]) -> bool {
         // Ensure the algorithm is the expected one.
         if self.algorithm != 1 {
             return false;
         }
         // Compute the expected signature and compare with the provided one.
         let expected = self.expected_signature(message);
         expected == signature
    }
}

fn run(checker: &dyn CryptoCheck, data: &[u8], sig: &[u8]) -> bool {
    // Process the message using the cryptographic checker.
    checker.verify(data, sig)
}

fn main() {
    // In the fixed version, only truly valid signatures are accepted.
    let worker = Worker { algorithm: 1 };
    let data = b"Important message";
    // For our simulation, a valid signature is the reversed message.
    let valid_sig = data.iter().rev().cloned().collect::<Vec<u8>>();
    if run(&worker, data, &valid_sig) {
         println!("Message accepted");
    } else {
         println!("Signature verification failed");
    }
}