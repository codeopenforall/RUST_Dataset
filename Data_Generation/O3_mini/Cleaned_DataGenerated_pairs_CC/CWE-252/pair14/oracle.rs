#[cfg(test)]
mod tests {
    use super::process_input;
    #[test]
    fn test_error_propagation() {
        let result = process_input(42);
        assert!(!result, "Expected false when operation fails, but got true");
    }
    #[test]
    fn test_success_case() {
        let result = process_input(10);
        assert!(result, "Expected true for successful operation.");
    }
}
