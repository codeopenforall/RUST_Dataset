//////////////////////////////
// Test Oracle Function
//////////////////////////////
// This test oracle is meant to be compiled and run separately from the vulnerable and fixed binaries.
// It ensures that for a given malicious input containing shell metacharacters, the output from the safe
// implementation strictly echoes the input (with a trailing newline) without interpreting any injected commands.
// In the vulnerable code, the shell would execute additional commands causing the output to differ and the test to fail.
#[cfg(test)]
mod tests {
    use super::Executor;

    #[test]
    fn test_safe_execution() {
        // Malicious input intended to perform command injection if unsanitized.
        let malicious = "malicious_input; echo injected".to_string();
        let executor = Executor::new(malicious.clone());
        let output = executor.run().expect("Execution failed");
        // Expected output: the echo command prints its argument followed by a newline.
        // In the fixed version, the output should be exactly the malicious input with a newline.
        let expected = format!("{}\n", malicious);
        assert_eq!(output, expected, "Vulnerability exploited: unexpected command injection detected.");
    }
}