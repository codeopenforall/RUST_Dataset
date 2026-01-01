/*
Test oracle function: This unit test calls the processing function with a malicious input string containing format specifiers.
For the vulnerable version, the unsafe use of the input causes undefined behavior (e.g., incorrect or unpredictable output),
and the test assertion (that the output exactly matches the raw input) fails.
For the fixed version, the user input is safely printed via the "%s" format string,
so the output exactly matches the input, and the test passes.

Note: This oracle function is independent from the main applications above.
*/

#[cfg(test)]
mod tests {
    use super::*;
    
    // Dummy process function signature re-imported from the code (vulnerable or fixed version).
    // For testing, ensure to call the same "process" function from the compiled version.
    #[test]
    fn test_format_handling() {
        // Malicious input that includes format specifiers.
        let input = "User input: %x %x";
        let output = process(input);
        // In the fixed version, output should simply be the input.
        // In the vulnerable version, embedded specifiers may lead to altered output.
        assert_eq!(output, input, "The processed output does not match the expected safe output");
    }
}