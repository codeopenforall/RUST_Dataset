#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn oracle() {
        let auth = Authenticator::new("SuperSecret");
        let result = auth.validate("BadToken");
        assert!(result.is_err(), "Authentication should fail for invalid token");
    }
}
