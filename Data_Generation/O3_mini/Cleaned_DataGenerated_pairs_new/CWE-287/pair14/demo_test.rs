#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_auth() {
        // Using the same struct as in both versions.
        let auth = Authenticator {
            valid: "supersecrettoken".to_string(),
        };
        // Exact match should pass.
        assert_eq!(auth.check("supersecrettoken"), true, "Exact match must pass");
        // A prefix token should fail in the secure (fixed) version.
        assert_eq!(auth.check("supersecret"), false, "Prefix token should not pass authentication");
    }
}