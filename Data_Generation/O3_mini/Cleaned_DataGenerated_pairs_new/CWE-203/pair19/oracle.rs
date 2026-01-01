#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_auth_service() {
        let svc = Service::new();
        let result = svc.authenticate("alice", "wrongpassword");
        let expected = "Authentication failed: Invalid username or password.";
        assert!(result.is_err(), "Authentication should fail for wrong credentials.");
        let err = result.unwrap_err();
        assert_eq!(err, expected, "The error message must be generic and not leak sensitive data.");
    }
}
