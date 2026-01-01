#[cfg(test)]
mod tests {
    use super::AuthManager;
    #[test]
    fn test_authentication_token() {
        let manager = AuthManager::new("SuperSecret");
        let malicious_token = "SuperSecretExtra";
        let result = manager.validate(malicious_token);
        assert!(
            !result,
            "Authentication should fail when extra characters are appended to the expected token."
        );
    }
}
