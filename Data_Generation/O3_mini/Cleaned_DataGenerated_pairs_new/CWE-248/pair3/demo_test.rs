///////////////////////// Test Oracle /////////////////////////
// This test ensures that when a negative value is supplied, the function returns an error.
// In the vulnerable version, the test would fail (i.e. panic) due to an uncaught exception,
// while in the fixed version, the error is caught and the test passes.

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_abnormal_input() {
        // For abnormal input (-1), the fixed implementation is expected
        // to catch and return an error rather than panicking.
        let result = run_task(-1);
        assert!(result.is_err(), "Expected an error for negative input");
    }
}