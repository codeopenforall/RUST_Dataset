//////////////////////////////////////////////////////////////
// Test Oracle - Should fail for the vulnerable version and pass for the fixed version.
//////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    // This test uses a bogus signature (64 bytes of zeros) which should be rejected.
    // The vulnerable version mistakenly accepts it, causing the test to fail.
    #[test]
    fn test_authentication() {
        let instance = Arc::new(Checker::new());
        let public_key: [u8; 32] = [1; 32];
        let message = b"Hello, World!";
        let bogus_signature: [u8; 64] = [0; 64];

        // Expected to be false for the secure/fixed version, and (incorrectly) true for the vulnerable version.
        let is_valid = instance.authenticate(&public_key, message, &bogus_signature);
        assert!(!is_valid, "Bogus signature should be rejected");
    }
}