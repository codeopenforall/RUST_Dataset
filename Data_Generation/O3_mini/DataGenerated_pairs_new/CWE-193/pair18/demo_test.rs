#[cfg(test)]
mod tests {
    use super::*;
    
    // The oracle function verifies that the processed vector has the correct length equal to the input slice.
    // For the vulnerable version, the length will be input.len() + 1, failing the invariant.
    // For the corrected code, the length will match input.len(), passing the test.
    #[test]
    fn test_process_length() {
        let handler = DataHandler;
        let input = vec![1, 2, 3, 4, 5];
        let result = handler.process(&input);
        // Assert the length is exactly the same as the input length.
        assert_eq!(result.len(), input.len(), "Processed vector has incorrect length");
    }
}