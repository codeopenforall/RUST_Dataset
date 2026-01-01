#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process_length() {
        let handler = DataHandler;
        let input = vec![1, 2, 3, 4, 5];
        let result = handler.process(&input);
        assert_eq!(result.len(), input.len(), "Processed vector has incorrect length");
    }
}
