//////////////////////////////
// Test Oracle Function
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::execute;

    #[test]
    fn test_oracle() {
        let result = execute();
        // The correct behavior is to detect that the machine is inactive.
        // For the fixed version, execute() returns an error.
        // For the vulnerable version, execute() erroneously returns Ok(value).
        assert!(result.is_err(), "Expected an error due to inactive machine, but got Ok({:?})", result);
        if let Err(e) = result {
            assert_eq!(e, "machine inactive", "Expected error 'machine inactive', got '{}'", e);
        }
    }
}