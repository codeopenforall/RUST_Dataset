#[cfg(test)]
mod tests {
    use std::sync::Arc;
    // Import the Executor struct from the current module.
    // In practice, compile the test with the fixed or vulnerable code independently.
    use super::Executor;

    // This oracle function simulates a scenario where malicious input is provided.
    // The expected safe behavior is that the output exactly mirrors the input,
    // whereas a vulnerable implementation might execute additional unintended commands.
    #[test]
    fn test_injection() {
        // Crafted malicious input containing a shell metacharacter.
        let malicious_input = "insecure; echo exploited".to_string();
        let executor = Executor::new(Arc::new(malicious_input.clone()));
        let output = executor.run().expect("Execution error");

        // Expected output for safe implementation: the literal input echoed with a newline.
        let expected = format!("{}{}", malicious_input, "\n");

        // This assertion enforces that the code does not interpret shell metacharacters.
        // For the vulnerable version, the output would include additional content (e.g., "exploited"),
        // causing the test to fail.
        assert_eq!(output, expected, "OS command injection vulnerability detected");
    }
}