#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_auth_message_consistency() {
        // Set up the repository with predefined credentials.
        let repo = Repository::new();
        
        // Attempt authentication with a valid username but incorrect password.
        let error_existing = repo.check("bob", "wrongpass").err().unwrap();
        
        // Attempt authentication with a non-existent username.
        let error_nonexisting = repo.check("mallory", "any").err().unwrap();
        
        // For the secure version, both errors must be identical to avoid information disclosure.
        assert_eq!(error_existing, "Invalid credentials", "Error message for existing account did not match the expected generic message");
        assert_eq!(error_nonexisting, "Invalid credentials", "Error message for non-existing account did not match the expected generic message");
    }
}