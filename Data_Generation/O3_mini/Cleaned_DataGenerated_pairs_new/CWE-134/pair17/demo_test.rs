////////////////////////////
// Test Oracle Function
////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    
    // This test oracle is intended to be used with both variants.
    // The safe version (fixed) produces an output exactly equal to the input.
    // The vulnerable version misinterprets the input format specifiers, so the output will differ.
    #[test]
    fn test_format_integrity() {
        let input = "Test literal: %s %x";
        let expected = "Test literal: %s %x";
        let proc = Processor::new();
        let output = proc.process(input);
        // For the safe version, the assertion should pass.
        // For the vulnerable version, the formatting function interprets the specifiers
        // and the output will not match the expected literal string.
        assert_eq!(output, expected, "The output should exactly match the literal input.");
    }
}