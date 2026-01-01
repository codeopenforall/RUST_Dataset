///////////////////////////////////////////////////////////////
// Test oracle: This test function verifies the behavior of the recursion parser.
// It constructs a deeply nested valid input (balanced parentheses) that exceeds the safe
// recursion limit established in the fixed version. The oracle expects that:
//   - In the vulnerable version, the uncontrolled recursion will eventually cause a panic due to stack overflow.
//   - In the fixed version, the function gracefully returns an error ("exceeded recursion limit") 
//     without panicking.
// The oracle uses catch_unwind to safely intercept panics.
///////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn test_recursion_limit() {
        // Construct a nested input with 1000 pairs of parentheses.
        let mut input = String::new();
        for _ in 0..1000 {
            input.push('(');
        }
        for _ in 0..1000 {
            input.push(')');
        }

        // Catch unwinding panics to detect stack overflows in the vulnerable version.
        let result = panic::catch_unwind(|| {
            process(&input)
        });

        // For the fixed version: process() should return Err("exceeded recursion limit") without panicking.
        // For the vulnerable version: process() is likely to cause a stack overflow and panic.
        assert!(result.is_ok(), "Function panicked due to uncontrolled recursion");
        let outcome = result.unwrap();
        assert_eq!(outcome, Err("exceeded recursion limit"), "Unexpected parsing outcome");
    }
}