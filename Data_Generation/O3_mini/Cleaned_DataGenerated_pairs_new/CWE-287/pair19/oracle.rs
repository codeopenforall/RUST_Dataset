#[cfg(test)]
mod tests {
    use super::*;
    fn perform_test<F: Fn(&str) -> bool>(auth_fn: F) {
        let test_token = "TopCow";
        let res = auth_fn(test_token);
        assert!(!res, "Authentication unexpectedly succeeded for token: {}", test_token);
    }
    #[test]
    fn test_authentication() {
        let validator = Validator::new();
        perform_test(|t| validator.check(t));
    }
}
