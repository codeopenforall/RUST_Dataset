#[cfg(test)]
mod tests {
    use super::process_data;
    #[test]
    fn test_input_validation() {
        let valid = process_data(8);
        assert!(valid.is_ok());
        assert_eq!(valid.unwrap(), "01234567");
        let malicious = process_data(12);
        assert!(malicious.is_err(), "Expected an error on input length exceeding buffer bounds");
    }
}
