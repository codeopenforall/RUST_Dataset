#[cfg(test)]
mod tests {
    use super::Authenticator;

    #[test]
    fn authentication_oracle() {
        // The correct secret is "SUPER_SECRET", so any other token must fail.
        let service = Authenticator::new("SUPER_SECRET");
        let invalid_token = "TEST_KEY";
        
        // The authentication check must reject an invalid token.
        // In the vulnerable implementation this test would fail because the token is erroneously accepted.
        assert!(
            !service.validate(invalid_token),
            "Authentication should fail for invalid token"
        );

        // Also, the proper token should succeed.
        let valid_token = "SUPER_SECRET";
        assert!(
            service.validate(valid_token),
            "Authentication should succeed for the valid token"
        );
    }
}