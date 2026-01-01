#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fencepost_error() {
        // Create a buffer of size 10.
        let buffer = DataBuffer::new(10);
        // The correct behavior is to return the last valid element (which is 9).
        let expected = 9;
        let result = buffer.process();
        // If the off-by-one error occurs, the returned result will be undefined (and likely not 9).
        assert_eq!(result, expected, "Test failed: off-by-one error detected in element access.");
    }
}