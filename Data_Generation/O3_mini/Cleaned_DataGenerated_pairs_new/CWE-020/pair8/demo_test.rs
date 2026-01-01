#[cfg(test)]
mod tests {
    use super::Info;
    use std::sync::Arc;

    // This test uses an invalid UTF-8 input (0xff, 0xfe, 0xfd).
    // The correct behavior is to reject the input and return an error.
    // In the vulnerable implementation, the function would erroneously succeed.
    // In the fixed implementation, it should return an Err.
    #[test]
    fn test_input_validation() {
        let handler = Arc::new(Info::new());
        let bad_input = vec![0xff, 0xfe, 0xfd];
        let result = handler.process_input(&bad_input);
        assert!(result.is_err(), "Expected error on invalid UTF-8 input");
    }
}