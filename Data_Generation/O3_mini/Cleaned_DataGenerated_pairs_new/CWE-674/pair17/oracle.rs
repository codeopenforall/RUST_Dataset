#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;
    #[test]
    fn test_recursive_limit() {
        let deep_input = "(".repeat(1200) + &")".repeat(1200);
        let vulnerable_result = panic::catch_unwind(|| {
            process(&deep_input).unwrap()
        });
        assert!(vulnerable_result.is_err(), "Vulnerable version should not handle deep recursion gracefully");
        let fixed_result = process(&deep_input);
        match fixed_result {
            Err(e) => assert_eq!(e, "Recursion depth limit exceeded", "Fixed version should report recursion limit"),
            Ok(_) => panic!("Fixed version should not parse input beyond max depth"),
        }
    }
}
