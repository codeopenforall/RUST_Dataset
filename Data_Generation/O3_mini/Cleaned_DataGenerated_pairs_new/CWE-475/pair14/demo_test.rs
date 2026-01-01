//////////////////////////// Test Oracle ////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_entry() {
        let block = MemoryBlock::new();
        // Valid parameters should return the correct sum.
        let valid = block.safe_entry(1, 3);
        assert_eq!(valid, Ok(90));

        // Invalid parameters: offset + count exceeds the buffer length.
        // In the fixed version, this should return an error.
        let invalid = block.safe_entry(2, 4);
        assert!(invalid.is_err(), "Expected error for invalid parameters");

        // For vulnerable code, the same test would either not return an error or could panic,
        // causing this test to fail.
    }
}
//////////////////////////// End of Test Oracle ////////////////////////////