//////////////////////////////////////////////
// Test Oracle Function
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::process_input;

    // This test is designed so that:
    // - With the vulnerable implementation, passing a format string with extra specifiers (e.g., "%x")
    //   would lead to unexpected output (or even a runtime issue), thus the test will fail.
    // - With the fixed implementation, the user input is treated as literal, and the output will match the expected string.
    #[test]
    fn test_process_input() {
        // Malicious input that includes format specifiers.
        // Expectation for fixed version: the output wraps the literal string.
        let input = "Malicious %x %x";
        let result = process_input(input);
        let expected = format!("User message: {}, Data: 42", input);
        // The fixed code should return the exact expected string.
        // With the vulnerable version, the formatting may interpret %x specifiers unexpectedly,
        // causing a mismatch.
        assert_eq!(result, expected, "The formatted output did not match the expected safe output.");
    }
}