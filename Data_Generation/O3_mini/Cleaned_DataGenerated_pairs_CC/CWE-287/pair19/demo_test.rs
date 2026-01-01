#[cfg(test)]
mod tests {
    // The test function verifies that the authentication mechanism refuses tokens that
    // do not exactly match the expected key ("TopSecret"). In the vulnerable code this test
    // will fail because tokens like "TopCow" pass due to only comparing the first three bytes.
    use super::*;

    // This helper function accepts a closure for performing token checks.
    fn perform_test<F: Fn(&str) -> bool>(auth_fn: F) {
        let test_token = "TopCow";
        let res = auth_fn(test_token);
        // The correct implementation should reject "TopCow".
        assert!(!res, "Authentication unexpectedly succeeded for token: {}", test_token);
    }

    #[test]
    fn test_authentication() {
        let validator = Validator::new();
        perform_test(|t| validator.check(t));
    }
}