///////////////////////////////////////
// Test Oracle Function
///////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::process::{Command, Stdio};
    use std::io::Read;

    // Expected complete string
    const EXPECTED: &str = "This is a test message that exceeds the typical struct size.";

    // Test the process() function directly.
    #[test]
    fn test_process_output() {
        let output = process();
        // The vulnerable version would produce a truncated string, so this test will fail for it,
        // whereas the fixed version produces the complete expected string.
        assert_eq!(output, EXPECTED, "The output string does not match the expected full string");
    }
}
///////////////////////////////////////
// End of Test Oracle Function
///////////////////////////////////////