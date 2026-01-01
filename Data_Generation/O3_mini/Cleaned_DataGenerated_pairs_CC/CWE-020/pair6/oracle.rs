#[cfg(test)]
mod tests {
    use super::Processor;
    #[test]
    fn test_input_length_validation() {
        let processor = Processor;
        let input = "10 hello";
        let result = processor.process(input);
        assert!(result.is_err(), "Expected an error due to length exceeding payload size");
        assert_eq!(result.unwrap_err(), "Length exceeds payload size");
    }
}
