#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_authentication_error_message() {
        let service = AuthService::new();
        let err = service.authenticate("bob", "any").unwrap_err();
        assert_eq!(err, "Invalid credentials", "Error message must be uniform to prevent information leakage");
    }
}
