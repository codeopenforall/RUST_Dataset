#[cfg(test)]
mod tests {
    use super::IdentityManager;
    use std::sync::Arc;
    #[test]
    fn test_authentication() {
        let manager = IdentityManager::new("S3cr3t".to_string());
        let malicious_token = "S3abcd";
        let result = manager.validate(malicious_token);
        assert!(!result, "Authentication bypass vulnerability detected.");
    }
}
