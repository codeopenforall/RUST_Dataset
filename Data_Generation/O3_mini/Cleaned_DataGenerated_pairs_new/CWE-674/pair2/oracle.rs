#[cfg(test)]
mod tests {
    use super::parse_nested;
    use std::panic;
    #[test]
    fn test_recursion_limit() {
        let deep_input = "(".repeat(1500) + &")".repeat(1500);
        let result = panic::catch_unwind(|| {
            parse_nested(&deep_input)
        });
        match result {
            Ok(inner) => {
                if let Ok(_) = inner {
                    panic!("Expected error due to recursion depth, but got a valid AST");
                }
            }
            Err(_) => {
                panic!("Test failed: uncontrolled recursion caused a panic (stack overflow)");
            }
        }
    }
}
