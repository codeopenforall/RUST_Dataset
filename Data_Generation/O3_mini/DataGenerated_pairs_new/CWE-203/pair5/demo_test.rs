//////////////////////////////////////////////
// Test Oracle for Authentication Module
//
// This oracle verifies that the authentication module 
// returns a uniform error message ("Invalid credentials.") 
// for all failed login attempts. When executed against the 
// vulnerable version, the test will fail due to error message 
// discrepancies, whereas the corrected version will pass.
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::LoginManager;

    #[test]
    fn test_authentication_uniform_error() {
        let manager = LoginManager::new();
        let err_existing = manager.validate("admin", "wrongpass").unwrap_err();
        let err_nonexisting = manager.validate("nonexistent", "anything").unwrap_err();
        // Assert that both error messages are exactly the same.
        assert_eq!(err_existing, "Invalid credentials.", "Error message for existing user is not uniform.");
        assert_eq!(err_nonexisting, "Invalid credentials.", "Error message for non-existing user is not uniform.");
    }
}