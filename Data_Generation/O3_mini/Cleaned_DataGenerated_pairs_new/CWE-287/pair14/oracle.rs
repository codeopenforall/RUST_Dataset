#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_token_auth() {
        let auth = Authenticator {
            valid: "supersecrettoken".to_string(),
        };
        assert_eq!(auth.check("supersecrettoken"), true, "Exact match must pass");
        assert_eq!(auth.check("supersecret"), false, "Prefix token should not pass authentication");
    }
}
