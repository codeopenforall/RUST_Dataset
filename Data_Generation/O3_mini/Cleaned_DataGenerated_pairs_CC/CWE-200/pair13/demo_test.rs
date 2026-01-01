//////////////// Test Oracle //////////////////
// This test oracle function is meant to be compiled and run separately from the above application code.
// It verifies that the error message returned from run_app() does NOT contain the sensitive key.
// For the vulnerable version, the test will fail because the secret "SuperSecretPassword" appears in the error message.
// For the corrected version, the test will pass.

#[cfg(test)]
mod tests {
    use super::run_app;

    #[test]
    fn test_secret_not_exposed() {
        let error_message = run_app();
        // The secret "SuperSecretPassword" should not appear in the error message.
        assert!(
            !error_message.contains("SuperSecretPassword"),
            "Sensitive information leaked in error message: {}",
            error_message
        );
    }
}