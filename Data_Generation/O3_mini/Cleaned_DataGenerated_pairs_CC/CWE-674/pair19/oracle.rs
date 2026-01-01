#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;
    #[test]
    fn test_deep_nested_input() {
        let mut input = String::new();
        for _ in 0..1500 {
            input.push('(');
        }
        for _ in 0..1500 {
            input.push(')');
        }
        let mut parser = Parser::new(&input);
        let result = panic::catch_unwind(panic::AssertUnwindSafe(|| parser.parse_expression()));
        match result {
            Ok(val) => {
                assert!(!val, "Expected failure due to recursion depth limit.");
            }
            Err(_) => {
                panic!("Parser panicked on deeply nested input, indicating uncontrolled recursion.");
            }
        }
    }
}
