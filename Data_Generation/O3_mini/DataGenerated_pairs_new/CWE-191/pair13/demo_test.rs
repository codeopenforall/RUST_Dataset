#[cfg(test)]
mod tests {
    use super::execute;
    
    // This test oracle expects that subtracting 20 from 10 will yield 0.
    // For the vulnerable code, the result will be a large number (e.g., 4294967286), causing the test to fail.
    #[test]
    fn test_underflow_prevention() {
        let result = execute(20);
        assert_eq!(result, 0, "Expected result is 0 after safe subtraction; vulnerability persists if not fixed.");
    }
}