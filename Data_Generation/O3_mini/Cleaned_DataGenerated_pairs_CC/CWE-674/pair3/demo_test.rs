#[cfg(test)]
mod tests {
    use super::*;

    // Oracle function that verifies the behavior using deeply nested input.
    #[test]
    fn recursion_limit_test() {
        // Create deeply nested input to trigger potential overflow.
        let deep_input = "(".repeat(1100) + &")".repeat(1100);

        // Test the version with uncontrolled recursion; expect a panic/stack overflow.
        let err = std::panic::catch_unwind(|| {
            let _ = execute(&deep_input).unwrap();
        });
        assert!(err.is_err(), "Expected failure in uncontrolled recursion scenario");

        // Test the secure version; it should gracefully error out with a controlled message.
        let result = execute(&deep_input);
        assert!(result.is_err(), "Expected controlled error due to recursion depth limit");
        if let Err(msg) = result {
            assert_eq!(msg, "Recursion depth limit exceeded", "Expected specific error message");
        }
    }
}