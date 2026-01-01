//////////////////////////////
// Test Oracle Function
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_exceptional_condition() {
        // When input is 25, the underlying operation times out.
        // The correct behavior is to propagate the Timeout error.
        let result = run_service(25);
        // For the vulnerable version this test would fail because it incorrectly returns Ok(0)
        // For the fixed version, it correctly returns Err(ErrKind::Timeout)
        assert!(
            matches!(result, Err(ErrKind::Timeout)),
            "Expected a timeout error but got: {:?}",
            result
        );
    }
}