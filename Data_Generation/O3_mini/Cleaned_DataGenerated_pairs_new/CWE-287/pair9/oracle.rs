#[cfg(test)]
mod tests {
    use super::AuthManager;
    #[test]
    fn test_authentication() {
        let auth = AuthManager::new();
        let token = "very".as_bytes();
        let result = auth.check(token.as_ptr(), token.len());
        assert_eq!(result, false, "Authentication should fail for a partial token");
    }
}
