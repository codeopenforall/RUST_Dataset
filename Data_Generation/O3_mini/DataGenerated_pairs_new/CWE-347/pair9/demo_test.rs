///////////////////////////////////////////////////////////
// Test Oracle Function
//
// This oracle function can be used in a unit test to verify 
// that the authentication mechanism correctly handles invalid
// signatures. When run against the vulnerable version, the test
// will fail because the unsafe bypass erroneously accepts the 
// invalid signature. In contrast, when run against the corrected 
// version, the test will pass as invalid signatures are properly rejected.
///////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    // This helper replicates the authentication process.
    fn authenticate(message: Vec<u8>, signature: Vec<u8>) -> bool {
        // For the purpose of the oracle, we use a fixed public key.
        let pubkey = vec![1, 2, 3, 4, 5];
        let checker = Arc::new(Mutex::new(Checker::new(pubkey)));
        run(checker, message, signature)
    }

    #[test]
    fn oracle_test() {
        // Prepare a message and deliberately invalid signature.
        let message = vec![10, 20, 30];
        // Choose a signature value that does not match the expected one.
        // For the fixed version, the expected signature would be:
        // (10+20+30+1+2+3+4+5) % 256 = (75) % 256 = 75 (as u8 value).
        // Selecting 0 (or any value != 75) should trigger the failure.
        let invalid_signature = vec![0];

        let result = authenticate(message, invalid_signature);
        // The test asserts that invalid signatures must be rejected.
        assert!(!result, "Authentication should fail for an invalid signature.");
    }
}