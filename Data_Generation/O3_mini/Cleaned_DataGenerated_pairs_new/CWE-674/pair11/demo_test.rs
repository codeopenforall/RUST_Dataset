#[cfg(test)]
mod tests {
    use std::panic;
    // Import the parse_nested function from the module containing the production code.
    use super::parse_nested;

    #[test]
    fn recursion_limit_test() {
        // Create deeply nested input: 1500 opening and closing parentheses.
        let depth = 1500;
        let input = "(".repeat(depth) + &")".repeat(depth);
        // Use catch_unwind to intercept a potential stack overflow panic.
        let result = panic::catch_unwind(|| {
            parse_nested(&input)
        });

        match result {
            // For the fixed code, the recursion limit is enforced so the function returns an error.
            Ok(inner_result) => {
                assert_eq!(inner_result, Err("Maximum recursion depth exceeded"),
                           "Expected error due to maximum recursion depth being exceeded");
            },
            // For the vulnerable code, the deep recursion triggers a panic (stack overflow).
            Err(_) => {
                panic!("Test failed: stack overflow occurred when processing deep input");
            }
        }
    }
}