////////////////////////////////////
// Test Oracle Function
////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    
    // This test is designed to verify that a header with an unauthorized origin is rejected.
    // For the vulnerable version, the assertion will fail (since it erroneously returns true),
    // whereas for the corrected version, the test passes.
    #[test]
    fn test_origin_validation() {
        let server = Server::new("example.com");
        // Input with a malicious origin that should be disallowed.
        let result = server.validate("malicious.com");
        // Explicit assertion: the return should be false for a rejected header.
        assert!(!result, "The header origin was not correctly validated and was accepted.");
    }
}