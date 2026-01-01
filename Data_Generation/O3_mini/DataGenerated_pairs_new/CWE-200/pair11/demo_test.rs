/////////////////////////////////////////////////////////////////
// Test Oracle: This unit test is designed to catch leakage of sensitive information.
// When executed against the vulnerable code, the test will fail because the error message contains
// the secret "SuperSecretPassword", whereas the fixed code properly sanitizes the message.
// The test simulates a processing failure and asserts that the secret does not appear in the error message.
/////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    // A helper function that triggers processing failure.
    fn simulate_failure() -> String {
        let config = Config { secret: "SuperSecretPassword".to_string() };
        let result = config.process(false);
        result.err().expect("Expected error on failure")
    }

    #[test]
    fn test_error_sanitization() {
        let err_msg = simulate_failure();
        // This assertion should fail in the vulnerable code because the error message will contain the secret.
        // In the fixed version, the error message is sanitized.
        assert!(
            !err_msg.contains("SuperSecretPassword"),
            "Error message contains leaked sensitive information"
        );
    }
}