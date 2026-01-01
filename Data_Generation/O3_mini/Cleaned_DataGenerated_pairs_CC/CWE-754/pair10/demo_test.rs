#[cfg(test)]
mod tests {
    use super::Engine;

    #[test]
    fn test_exception_condition() {
        let engine = Engine;
        // We expect the corrected implementation to return an error due to a timeout.
        let result = engine.execute();
        // The fixed version will produce an Err("Operation timed out") when timeout occurs.
        assert!(result.is_err(), "Expected an error due to timeout, but got a success: {:?}", result);
        assert_eq!(result.unwrap_err(), "Operation timed out", "The error message did not match");
    }
}