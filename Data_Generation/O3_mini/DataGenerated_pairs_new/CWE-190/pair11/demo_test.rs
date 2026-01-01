/////////////////////////////////////////////////////////////
// Test Oracle: This unit test calls the simulation function and
// asserts that an integer overflow is detected. In the vulnerable
// implementation, the simulation would complete and yield a wrapped
// balance (an Ok value), causing the test to fail. In the corrected
// version, the checked addition detects the overflow and returns
// an Err, making the test pass.
/////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overflow_detection() {
        let result = simulate_transactions();
        // The expected behavior is to detect overflow and return an error.
        assert!(result.is_err(), "Expected error due to integer overflow, got {:?}", result);
    }
}