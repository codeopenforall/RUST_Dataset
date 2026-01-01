////////////////////////////////////////////////////////////
// Test Oracle Function for Vulnerability Validation
////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::Processor;

    // The test passes for the corrected version because it will return an error when:
    //  - The specified length exceeds the actual payload length.
    // For the vulnerable version, the unsafe block might produce unpredictable results,
    // causing the test to fail consistently.
    #[test]
    fn test_input_length_validation() {
        let processor = Processor;
        // Input where the expected length (10) is greater than the payload length ("hello" is 5).
        let input = "10 hello";
        let result = processor.process(input);
        // For the fixed version, we expect an explicit error message.
        assert!(result.is_err(), "Expected an error due to length exceeding payload size");
        assert_eq!(result.unwrap_err(), "Length exceeds payload size");
    }
}
////////////////////////////////////////////////////////////
// End of Test Oracle Function
////////////////////////////////////////////////////////////