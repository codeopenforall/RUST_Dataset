--------------------------------------------------
#[cfg(test)]
mod tests {
    use super::run_processing;

    // This oracle test function calls run_processing with an oversized input.
    // For the vulnerable implementation, the unchecked error will cause the function
    // to erroneously return Ok(()), and the assertion will fail.
    // For the fixed implementation, the function correctly returns an Err.
    #[test]
    fn test_error_propagation() {
        let bad_input = vec![1u8; 2048];
        let result = run_processing(&bad_input);
        // We expect an error for oversized input.
        assert!(result.is_err(), "The processing function should fail on oversized input.");
    }
}
--------------------------------------------------