#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    // The test case uses an intentionally malformed signature to demonstrate the vulnerability.
    // In the insecure implementation, the unsafe code “completes” the missing byte and erroneously accepts the signature,
    // whereas in the corrected implementation, the check for exact signature length causes verification to fail.
    #[test]
    fn test_oracle() {
        // Malicious input: a signature with 63 bytes instead of 64.
        let bad_sig = vec![1u8; 63];
        let doc = Data::new(vec![0u8; 10], bad_sig);
        let shared = Arc::new(Mutex::new(doc));
        let result = process(shared);
        // The expected result is false (i.e. rejection), as the signature is malformed.
        assert_eq!(result, false, "Verification should fail when signature length is not exactly 64 bytes");
    }
}