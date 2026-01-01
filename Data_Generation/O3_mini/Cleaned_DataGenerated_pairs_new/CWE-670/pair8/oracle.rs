#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_security_validation() {
        let processor = Processor::new(50);
        let result = processor.process(20);
        assert!(result.is_err(), "Unauthorized access granted when it should be denied.");
        if let Err(err) = result {
            assert_eq!(err, "Access Denied");
        }
    }
}
