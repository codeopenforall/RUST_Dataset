#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_authentication() {
        let auth = Authenticator::new("Password".to_string());
        assert_eq!(auth.process("P12345"), false, "Authentication should fail for weak token input");
        assert_eq!(auth.process("Password"), true, "Authentication should succeed for correct token input");
    }
}
