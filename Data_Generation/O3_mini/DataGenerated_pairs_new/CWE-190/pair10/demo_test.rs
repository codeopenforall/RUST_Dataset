#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_oracle() {
        // The correct result should be:
        // (u32::MAX as u64 + 1) * 2 = (4294967295 + 1)*2 = 8589934592.
        let expected: u64 = 8589934592;
        let result = run_calc();
        assert_eq!(result, expected, "Test failed: expected {} but got {}", expected, result);
    }
}