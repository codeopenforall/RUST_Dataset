#[cfg(test)]
mod tests {
    use super::*;

    // Test function used as test oracle.
    // For input values that trigger an overflow (50000 * 50000), the fixed version returns None.
    // The vulnerable version would return Some(wrapped_value), causing the test to fail.
    #[test]
    fn validate_overflow_detection() {
        let calc = Engine;
        let result = calc.compute(50000, 50000);
        // The expected behavior is to get None if overflow is detected.
        assert!(result.is_none(), "Expected overflow detection (None), but got {:?}", result);
    }
}