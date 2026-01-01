///////////////////////////////////////////////////////
// Test Oracle Function for Authentication Check
// This test asserts that on a failed login attempt,
// the error message is uniformly "invalid credentials".
// In the vulnerable version, different messages would be returned,
// causing this test to fail.
///////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_auth_error_message() {
        let auth = Auth::new();
        // Trigger log in with a known username and incorrect password.
        let result = auth.login("alice", "wrong_password");
        // Expect the fixed behavior: uniform error message.
        assert!(result.is_err());
        let err_msg = result.err().unwrap();
        assert_eq!(err_msg, "invalid credentials");
    }
}