////////////////////////////////////////////////////////////
// Test Oracle for Authentication Check
//
// This test function is designed to verify that an invalid token is always
// rejected, even if a debug mode might be flagged in the insecure version.
// When run against the vulnerable implementation, the test will fail because
// the debug flag improperly bypasses authentication. The corrected version
// correctly denies access for an invalid token.
////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_admin_access() {
        // When debug mode is set to true, the vulnerable version would incorrectly
        // grant access even with an invalid token.
        let invalid_token = "invalid_token";
        let server = Server::new(true);
        let result = server.handle_admin(invalid_token);
        // The expectation is that access is denied (false) regardless of debug mode.
        assert!(!result, "Invalid credentials should not grant access");
    }
}