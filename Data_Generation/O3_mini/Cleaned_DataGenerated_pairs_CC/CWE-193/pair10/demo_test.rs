//////////////////////////////////////////////////////////////
// Test Oracle Function
// This test function invokes the 'run' function with a known input and asserts
// that the output's length and content match the expected values. The test is designed
// to fail against the vulnerable implementation (which returns a vector missing the last byte)
// and pass with the corrected version.
//////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_processing() {
        let input = b"abcdef"; // Expected 6 bytes.
        let output = run(input);
        // Assert the output length equals the input length.
        assert_eq!(output.len(), input.len(), "Output length mismatch");
        // Assert that the content exactly matches the input.
        assert_eq!(output, input.to_vec(), "Output content mismatch");
    }
}