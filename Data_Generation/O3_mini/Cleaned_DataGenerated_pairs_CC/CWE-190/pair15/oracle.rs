#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_overflow_behavior() {
        let input = 50000;
        let processor = Processor;
        let result = processor.dispatch(input);
        let expected_fixed = i32::MAX;
        assert_eq!(result, expected_fixed, "Test failed: the safe version should detect overflow and saturate to i32::MAX");
    }
}
