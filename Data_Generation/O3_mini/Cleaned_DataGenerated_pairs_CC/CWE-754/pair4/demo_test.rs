//////////////////////////////
// Test Oracle Function
// This test fails for the vulnerable version (which incorrectly returns Ok(0) on a timeout)
// and passes for the fixed version (which returns an error on timeout).
//////////////////////////////

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_timeout_condition() {
        // Use input=1, which triggers a sleep that exceeds the recv_timeout duration.
        let result = process(1);
        // For a proper implementation, a timeout should yield an error.
        assert!(
            result.is_err(),
            "Expected an error due to timeout, but got a successful result."
        );
        if let Err(err) = result {
            assert_eq!(err, "Operation timed out", "Unexpected error message.");
        }
    }
}