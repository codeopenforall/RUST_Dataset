#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    // This test verifies that invalid credentials are rejected.
    #[test]
    fn test_authentication() {
        // Force a production-like environment to enforce secure verification.
        env::set_var("APP_MODE", "production");
        let acc = Account::new("user", "wrongpassword");
        let result = validate(&acc);
        // In secure (fixed) flow, invalid credentials must return false.
        assert_eq!(result, false, "Authentication bypass detected: invalid credentials should not authenticate.");
    }
}