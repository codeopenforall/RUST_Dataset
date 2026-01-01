#[cfg(test)]
mod tests {
    use super::auth::Identifier;
    #[test]
    fn test_authentication() {
        let checker = Identifier::new("secret");
        let result = checker.verify("skip_this");
        assert!(!result, "Authentication incorrectly accepted an invalid token");
    }
}
