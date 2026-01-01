#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_auth_message_consistency() {
        let repo = Repository::new();
        let error_existing = repo.check("bob", "wrongpass").err().unwrap();
        let error_nonexisting = repo.check("mallory", "any").err().unwrap();
        assert_eq!(error_existing, "Invalid credentials", "Error message for existing account did not match the expected generic message");
        assert_eq!(error_nonexisting, "Invalid credentials", "Error message for non-existing account did not match the expected generic message");
    }
}
