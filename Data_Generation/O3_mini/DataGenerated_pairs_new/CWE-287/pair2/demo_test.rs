/////////////////////////////////////////////
// Test Oracle for Authentication
// This test oracle can be compiled and run separately 
// to validate that the authentication function in the 
// fixed version correctly rejects an invalid token while 
// the vulnerable version would erroneously succeed.
/////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oracle() {
        // Create an authenticator initialized with the secret key.
        let auth = Authenticator::new("SuperSecret");
        // Provide an incorrect token.
        let result = auth.validate("BadToken");
        // In the fixed implementation, this assertion will pass
        // because the invalid token is properly rejected.
        assert!(result.is_err(), "Authentication should fail for invalid token");
    }
}