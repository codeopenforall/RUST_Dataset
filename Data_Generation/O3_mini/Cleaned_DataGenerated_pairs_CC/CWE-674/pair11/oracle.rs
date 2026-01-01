#[cfg(test)]
mod tests {
    use std::panic;
    use super::parse_nested;
    #[test]
    fn recursion_limit_test() {
        let depth = 1500;
        let input = "(".repeat(depth) + &")".repeat(depth);
        let result = panic::catch_unwind(|| {
            parse_nested(&input)
        });
        match result {
            Ok(inner_result) => {
                assert_eq!(inner_result, Err("Maximum recursion depth exceeded"),
                           "Expected error due to maximum recursion depth being exceeded");
            },
            Err(_) => {
                panic!("Test failed: stack overflow occurred when processing deep input");
            }
        }
    }
}
