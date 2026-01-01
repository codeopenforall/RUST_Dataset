#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_oracle() {
        let handler = DataHandler::new(vec![100, 200, 300, 400]);
        let result = handler.get_item(4);
        assert!(
            result.is_err(),
            "Expected error for out-of-bound access, but got: {:?}",
            result
        );
    }
}
