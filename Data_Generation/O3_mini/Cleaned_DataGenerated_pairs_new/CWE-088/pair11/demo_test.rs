//////////////////////////////////////////////////////////////
// Test Oracle Function for OS Command Injection Vulnerability
//
// This test checks that when provided with an injection string, the output from the command
// execution function equals the expected safe behavior only in the corrected code.
// In the vulnerable version, the injected shell metacharacter would be interpreted, resulting in
// additional or modified output, causing the test to fail.
//////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::execute_command;

    #[test]
    fn test_injection_prevention() {
        // This input attempts to inject an extra command ("echo hacked").
        let injection_input = "hello; echo hacked";
        // Expected output for a safe implementation:
        // The echo command should treat the entire input as a literal argument.
        let expected_output = "hello; echo hacked\n";
        
        let output = execute_command(injection_input)
            .expect("Failed to execute command");
        
        // The assertion will fail for the vulnerable version if the shell executes the injected command.
        assert_eq!(
            output, expected_output,
            "Test failed: Detected command injection vulnerability"
        );
    }
}