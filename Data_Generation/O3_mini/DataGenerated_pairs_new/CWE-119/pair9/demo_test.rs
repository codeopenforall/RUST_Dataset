//////////////////////////////////////////////
// Test Oracle: Verifying Memory Safety
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    // Import the processing function; for the vulnerable build this will be the unsafe version,
    // for the corrected build the version with proper bounds checking.
    use super::do_process;

    #[test]
    fn test_memory_bounds() {
        // Triggering input: 20 bytes, which is above the internal buffer capacity of 16 bytes.
        let input = vec![1u8; 20];

        // The correct behavior is to reject the oversized input and return an error.
        let result = do_process(&input);

        // Assert that an error is returned; if we receive a valid sum, the vulnerability is present.
        assert!(result.is_err(), "Expected error on oversized input, but got a valid result");
    }
}