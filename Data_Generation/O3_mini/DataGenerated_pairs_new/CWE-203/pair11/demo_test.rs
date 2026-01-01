//////////////////////////////////////////
// Test Oracle for Authentication Module
//////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    
    // This test expects the error message to be uniform ("Invalid credentials")
    // If the implementation reveals different error messages (e.g., "User does not exist"),
    // the test will fail.
    #[test]
    fn test_authentication_error_message() {
        let service = AuthService::new();
        // Attempt authentication with a non-existent user.
        let err = service.authenticate("bob", "any").unwrap_err();
        assert_eq!(err, "Invalid credentials", "Error message must be uniform to prevent information leakage");
    }
}