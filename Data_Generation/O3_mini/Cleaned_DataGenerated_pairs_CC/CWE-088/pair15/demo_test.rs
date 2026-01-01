//////////////////////////////
// Test Oracle Function
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    
    // This test simulates a malicious payload designed to inject an extra command.
    // The vulnerable code (if exploited) will execute that extra command, resulting in output
    // that includes the text "injected" on a new line. The secure version will treat the entire
    // string as a literal argument, so "injected" will not produce a new line separated output.
    #[test]
    fn test_injection() {
        // Malicious input containing a command separator and an extra command.
        let malicious_input = "test_payload; echo injected";
        let app = Application::new(malicious_input);
        let output = app.start().expect("Command execution failed");
        
        // In the vulnerable version, the shell interprets the semicolon as a command separator,
        // so the output would contain a newline (i.e., two separate outputs).
        // The test asserts that the output should not contain a newline followed by "injected".
        // The fixed version, which passes the untrusted input safely, will not create an extra line.
        assert!(
            !output.contains("\ninjected"),
            "Test failed: Injection occurred due to improper command handling"
        );
    }
}