/////////////////////////////////////////////////////////////
// Test Oracle Function (separate from the main programs)
// This test should fail for the vulnerable version and pass for the corrected version.
/////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    // Import the process function from the module under test.
    // In the vulnerable build, this test is expected to fail due to unsafe formatting.
    // In the corrected build, the test will pass.
    use super::process;

    #[test]
    fn test_format_safety() {
        // Malicious input containing format specifiers.
        let input = "malicious %x %x %x";
        // In the fixed version, the output should exactly echo the malicious input.
        let expected = "malicious %x %x %x";
        let output = process(input);
        assert_eq!(output, expected, "Output does not match expected safe formatting behavior");
    }
}