#[cfg(test)]
mod tests {
    use super::*;
    // Test oracle for detecting command injection vulnerability.
    #[test]
    fn test_injection_protection() {
        // Malicious input attempting to inject an extra command.
        let malicious = "safe_input; echo injected";
        let executor = Executor::new(malicious);
        let result = executor.run();

        // In a vulnerable implementation, the injected command would execute,
        // resulting in "injected" appearing in the output.
        // In the corrected implementation, the input is rejected with an error.
        match result {
            Ok(output) => {
                assert!(
                    !output.contains("injected"),
                    "Injection succeeded; vulnerability present."
                );
            }
            Err(err) => {
                assert_eq!(
                    err, "Invalid characters in input",
                    "Unexpected error message."
                );
            }
        }
    }
}