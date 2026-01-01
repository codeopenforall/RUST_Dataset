#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fencepost_error() {
        let buffer = DataBuffer::new(10);
        let expected = 9;
        let result = buffer.process();
        assert_eq!(result, expected, "Test failed: off-by-one error detected in element access.");
    }
}
