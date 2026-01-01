#[cfg(test)]
mod tests {
    use super::AuthService;
    #[test]
    fn authentication_oracle() {
        let auth = AuthService { valid_token: "secure123".to_string() };
        unsafe {
            assert_eq!(
                auth.validate("test"),
                false,
                "Oracle: Token 'test' should be rejected"
            );
        }
        unsafe {
            assert_eq!(
                auth.validate("secure123"),
                true,
                "Oracle: Token 'secure123' should be accepted"
            );
        }
    }
}
