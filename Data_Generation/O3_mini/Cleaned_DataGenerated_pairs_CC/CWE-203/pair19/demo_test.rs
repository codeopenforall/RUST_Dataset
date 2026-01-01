#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_service() {
        // Instantiate the authentication service.
        let svc = Service::new();

        // Test with an incorrect password.
        let result = svc.authenticate("alice", "wrongpassword");

        // The expected error message for a correct, generic error response.
        let expected = "Authentication failed: Invalid username or password.";

        // The vulnerable version would include the expected password in the error message, so this test will fail.
        // The fixed version returns a uniform error message.
        assert!(result.is_err(), "Authentication should fail for wrong credentials.");
        let err = result.unwrap_err();
        assert_eq!(err, expected, "The error message must be generic and not leak sensitive data.");
    }
}