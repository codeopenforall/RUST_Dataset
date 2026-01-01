///////////////////////////////////////////////////////////
// Test Oracle Function
// This test verifies that when using the safe implementation:
// Given an input with shell metacharacters ("safe; echo injected"),
// the output should be exactly "safe; echo injected\n".
// In the vulnerable implementation, the shell would interpret the semicolon and execute an extra command,
// leading to a different output (e.g. "safe\ninjected\n") causing the test to fail.
///////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    // It is assumed that the crate exposes a public function `process(&str) -> String`
    // from the implementation under test.
    use super::process;
    
    #[test]
    fn test_command_execution() {
        let input = "safe; echo injected";
        // Expected output for the safe implementation: the argument is echoed literally.
        let expected = "safe; echo injected\n";
        let output = process(input);
        assert_eq!(output, expected, "Command execution output mismatch");
    }
}

// Additionally, here's a standalone oracle function that can be used for manual verification.
pub fn test_oracle() {
    let input = "safe; echo injected";
    let expected = "safe; echo injected\n";
    let output = process(input);
    assert_eq!(output, expected, "Command execution output mismatch");
}