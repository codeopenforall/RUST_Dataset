///////////////////////////////////////////////////////////////
// Test Oracle Function for Authentication Bypass Vulnerability
///////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_authentication() {
        // For the vulnerable code, setting "DEBUG_MODE" to "1" will incorrectly bypass authentication.
        // For the fixed code, the bypass is removed so this should fail authentication.
        env::set_var("DEBUG_MODE", "1");
        
        let manager = AuthManager::new();
        // The authentication check for invalid credentials ("user", "wrongpass")
        // MUST fail, i.e. return false. In the vulnerable version, it would erroneously return true.
        assert!(!manager.check("user", "wrongpass"), "Authentication bypass detected: invalid credentials should not be accepted");
    }
}