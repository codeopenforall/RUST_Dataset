#[cfg(test)]
mod tests {
    use super::AuthService;
    #[test]
    fn test_auth_error_message() {
        let service = AuthService::new();
        let result = service.login_auth("alice", "wrongpassword");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid credentials");
    }
}
