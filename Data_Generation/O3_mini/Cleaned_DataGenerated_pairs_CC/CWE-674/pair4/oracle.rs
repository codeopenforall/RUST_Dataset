#[cfg(test)]
mod tests {
    use std::panic;
    use crate::parser_run;
    #[test]
    fn recursion_limit_oracle() {
        let deep_input = "(".repeat(1500) + &")".repeat(1500);
        let result = panic::catch_unwind(|| {
            parser_run(&deep_input)
        });
        assert!(result.is_ok(), "Program panicked due to uncontrolled recursion");
        let parse_result = result.unwrap();
        assert!(parse_result.is_err(), "Expected an error due to excessive recursion depth");
        assert_eq!(parse_result.unwrap_err(), "Maximum recursion depth reached");
    }
}
