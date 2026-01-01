/*
   The oracle function validates the operation's correctness in both implementations.
   It invokes process_input with the critical input value (42) that causes unsafe_op to return an error.

   For the vulnerable version, due to the unchecked return value, the flag is set to true, causing the test to fail.
   For the fixed version, the flag remains false when an error occurs, so the test passes.

   This function should be used in a separate test harness.
*/
#[cfg(test)]
mod tests {
    use super::process_input;

    #[test]
    fn test_error_propagation() {
        // For critical input that triggers the error.
        let result = process_input(42);
        // In the secure (fixed) version, result should be false.
        assert!(!result, "Expected false when operation fails, but got true");
    }

    #[test]
    fn test_success_case() {
        // For an input that does not trigger the error.
        let result = process_input(10);
        // For both vulnerable and fixed version, a successful operation should yield true.
        assert!(result, "Expected true for successful operation.");
    }
}