#[cfg(test)]
mod tests {
    use super::execute;
    #[test]
    fn test_oracle() {
        let result = execute();
        assert!(result.is_err(), "Expected an error due to inactive machine, but got Ok({:?})", result);
        if let Err(e) = result {
            assert_eq!(e, "machine inactive", "Expected error 'machine inactive', got '{}'", e);
        }
    }
}
