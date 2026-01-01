#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn recursion_limit_test() {
        let deep_input = "(".repeat(1100) + &")".repeat(1100);
        let err = std::panic::catch_unwind(|| {
            let _ = execute(&deep_input).unwrap();
        });
        assert!(err.is_err(), "Expected failure in uncontrolled recursion scenario");
        let result = execute(&deep_input);
        assert!(result.is_err(), "Expected controlled error due to recursion depth limit");
        if let Err(msg) = result {
            assert_eq!(msg, "Recursion depth limit exceeded", "Expected specific error message");
        }
    }
}
