--------------------------------------------------
#[cfg(test)]
mod tests {
    use super::run_processing;
    #[test]
    fn test_error_propagation() {
        let bad_input = vec![1u8; 2048];
        let result = run_processing(&bad_input);
        assert!(result.is_err(), "The processing function should fail on oversized input.");
    }
}
--------------------------------------------------
