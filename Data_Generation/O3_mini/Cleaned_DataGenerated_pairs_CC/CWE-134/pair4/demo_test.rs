//////////////////////////////////////////////////////////////
// Test Oracle for verifying against CWE-134
// This unit test function is designed to run separately against the compiled binary.
// When supplied with the triggering input "%s", the safe implementation should produce
// exactly "%s" as output, whereas the vulnerable version may not.
//
// To simulate the test, this oracle calls the public function "process" with a
// known input and asserts the expected output.
// Note: This test oracle should be compiled in a separate test module.
//////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_format_handling() {
        // Triggering input that could be dangerous if improperly handled.
        let input = "%s";
        let result = process(input);
        // In the secure case, the output should be exactly "%s".
        // For the insecure code, the behavior is unpredictable.
        assert_eq!(result, "%s", "The output does not match the safe expected value");
    }
}