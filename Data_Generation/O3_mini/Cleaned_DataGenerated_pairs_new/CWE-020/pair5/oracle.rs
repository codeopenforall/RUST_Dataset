#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;
    #[test]
    fn test_oracle() {
        let handler = Handler::new();
        let malicious_input = "95";
        let result = panic::catch_unwind(|| handler.execute(malicious_input));
        match result {
            Ok(Ok(_)) => panic!("Test failed: Expected error on out-of-bounds input, but execution succeeded."),
            Ok(Err(e)) => assert_eq!(e, "index out of bounds", "Expected specific error message on out-of-bounds input."),
            Err(_) => panic!("Test failed: Execution panicked on out-of-bounds input."),
        }
    }
}
