#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;
    #[test]
    fn test_oracle() {
        let result = panic::catch_unwind(|| {
            run(); 
        });
        assert!(result.is_err(), "Test failed: operation error was ignored");
    }
}
