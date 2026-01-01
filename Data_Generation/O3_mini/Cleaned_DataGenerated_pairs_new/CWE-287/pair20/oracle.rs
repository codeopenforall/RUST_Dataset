fn test_authentication() {
    let auth = Authenticator::new();
    let result = auth.check("invalid_token");
    assert!(result.is_err(), "Authentication should fail for an invalid credential.");
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn oracle() {
        test_authentication();
    }
}
