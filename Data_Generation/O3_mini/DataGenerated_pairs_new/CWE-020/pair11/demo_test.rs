#[cfg(test)]
mod tests {
    use super::Processor;

    #[test]
    fn validate_input_length() {
        // The provided length exceeds the buffer size.
        let invalid_length = 30; // "Sensitive buffer data" is 21 bytes long.
        let processor = Processor::new();
        // For the fixed version, the call should return an error.
        // For the insecure version, it erroneously returns Ok(..) or undefined behavior.
        let result = processor.process(invalid_length);
        assert!(result.is_err(), "Expected an error for invalid input length");
    }
}