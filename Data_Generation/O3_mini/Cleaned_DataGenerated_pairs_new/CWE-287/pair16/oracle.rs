#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    fn create_manager() -> AuthManager {
        AuthManager::new()
    }
    #[test]
    fn test_authentication() {
        let manager = create_manager();
        assert!(manager.verify("valid_token123"), "Valid token must pass authentication");
        assert!(!manager.verify("v"), "Incorrect token must fail authentication");
    }
}
