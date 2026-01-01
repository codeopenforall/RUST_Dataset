///////////////////////////////////////////////
// Test Oracle Function
///////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::panic;
    // Assume the common parser_run function is available from the module.
    // The same function is used by both versions.
    use crate::parser_run;

    #[test]
    fn recursion_limit_oracle() {
        // Create a deeply nested input that would trigger uncontrolled recursion.
        let deep_input = "(".repeat(1500) + &")".repeat(1500);
        let result = panic::catch_unwind(|| {
            parser_run(&deep_input)
        });
        // In the vulnerable version, this input causes a stack overflow (panic),
        // so catch_unwind would capture an error. In the fixed version, the panic
        // is avoided and the parser returns a controlled error.
        assert!(result.is_ok(), "Program panicked due to uncontrolled recursion");
        let parse_result = result.unwrap();
        // The contract is that deeply nested input should result in a controlled error.
        assert!(parse_result.is_err(), "Expected an error due to excessive recursion depth");
        assert_eq!(parse_result.unwrap_err(), "Maximum recursion depth reached");
    }
}