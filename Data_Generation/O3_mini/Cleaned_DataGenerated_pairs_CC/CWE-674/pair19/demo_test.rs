#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    // This test is designed as an oracle: when run against the safe variant it passes,
    // but with the uncontrolled (vulnerable) variant it will either panic (stack overflow)
    // or not return a graceful failure.
    #[test]
    fn test_deep_nested_input() {
        // Craft deliberately deep nested input (beyond the safe limit)
        let mut input = String::new();
        // 1500 is chosen to exceed the depth limit in the safe version (MAX_DEPTH=1000)
        for _ in 0..1500 {
            input.push('(');
        }
        for _ in 0..1500 {
            input.push(')');
        }
        let mut parser = Parser::new(&input);
        // Catch unwind to handle potential stack overflows in the vulnerable variant.
        let result = panic::catch_unwind(panic::AssertUnwindSafe(|| parser.parse_expression()));
        match result {
            Ok(val) => {
                // In the fixed code, excessive nesting should be detected and return false.
                assert!(!val, "Expected failure due to recursion depth limit.");
            }
            Err(_) => {
                panic!("Parser panicked on deeply nested input, indicating uncontrolled recursion.");
            }
        }
    }
}