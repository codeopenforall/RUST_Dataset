#[cfg(test)]
mod tests {
    use std::thread;
    use super::DataHandler;
    #[test]
    fn test_trailing_zero() {
        let handler = DataHandler;
        let input = b"hello";
        let result = handler.combine(input);
        assert_eq!(result[result.len() - 1], 0, "The trailing element should be 0");
    }
}
