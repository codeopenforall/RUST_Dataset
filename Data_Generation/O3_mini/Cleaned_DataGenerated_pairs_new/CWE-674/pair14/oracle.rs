#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;
    #[test]
    fn test_recursion_limit() {
        let mut input = String::new();
        for _ in 0..1000 {
            input.push('(');
        }
        for _ in 0..1000 {
            input.push(')');
        }
        let result = panic::catch_unwind(|| {
            process(&input)
        });
        assert!(result.is_ok(), "Function panicked due to uncontrolled recursion");
        let outcome = result.unwrap();
        assert_eq!(outcome, Err("exceeded recursion limit"), "Unexpected parsing outcome");
    }
}
