#[cfg(test)]
mod tests {
    use super::Processor;
    #[test]
    fn validate_input_length() {
        let invalid_length = 30; 
        let processor = Processor::new();
        let result = processor.process(invalid_length);
        assert!(result.is_err(), "Expected an error for invalid input length");
    }
}
