///////////////////////////////////////////////////////////////////////////////
// Test Oracle Function
// This function serves as an automated test to compare the outputs from the vulnerable
// and fixed implementations. The expected output is "hello" (ignoring extraneous whitespace).
// When executed against the vulnerable version, the test should fail (e.g., due to race
// conditions or unexpected modifications in the search PATH).
///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    // A helper function representing the contract: executing the command should produce "hello".
    fn contract_output<F: Fn() -> Result<String, String>>(executor: F) {
        let output = executor().unwrap_or_else(|err| err);
        // Trim whitespace and newline characters.
        let trimmed = output.trim();
        assert_eq!(trimmed, "hello", "The command output did not match the expected value.");
    }

    #[test]
    fn test_insecure() {
        // For the vulnerable version, due to the race and unsafe environment handling,
        // the test is expected to fail (i.e. output is not reliable).
        // This failure demonstrates the vulnerability.
        let result = Executor::launch();
        if let Ok(out) = result {
            assert_ne!(out.trim(), "hello", "Vulnerable version should not reliably output 'hello'.");
        } else {
            // In case of error, consider it as failed test for safe behavior.
            assert!(true, "Vulnerable version errored as expected.");
        }
    }

    #[test]
    fn test_secure() {
        // For the secure version, using the fixed trusted PATH, the output must always be "hello".
        let result = SafeExecutor::launch();
        let output = result.expect("Secure execution failed unexpectedly.");
        assert_eq!(output.trim(), "hello", "Secure version did not output the expected value.");
    }
}