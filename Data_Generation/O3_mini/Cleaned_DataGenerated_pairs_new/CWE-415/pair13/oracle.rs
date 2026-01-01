#[cfg(test)]
mod tests {
    use super::run;
    use std::panic;
    #[test]
    fn test_oracle() {
        let result = panic::catch_unwind(|| run());
        assert!(result.is_ok(), "The function panicked, indicating a double free vulnerability");
        let value = result.expect("Expected a successful result");
        assert_eq!(value, 20, "Unexpected computation result");
    }
}
