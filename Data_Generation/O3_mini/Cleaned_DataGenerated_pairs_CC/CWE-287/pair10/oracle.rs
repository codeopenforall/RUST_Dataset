#[cfg(test)]
mod tests {
    use super::Authenticator;
    #[test]
    fn authentication_oracle() {
        let service = Authenticator::new("SUPER_SECRET");
        let invalid_token = "TEST_KEY";
        assert!(
            !service.validate(invalid_token),
            "Authentication should fail for invalid token"
        );
        let valid_token = "SUPER_SECRET";
        assert!(
            service.validate(valid_token),
            "Authentication should succeed for the valid token"
        );
    }
}
