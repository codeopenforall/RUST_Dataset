////////////////// Test Oracle //////////////////
// This test oracle is separate from the application code. It calls the run() function
// with a triggering input (0) and asserts that the returned error message equals "Zero size".
// In the fixed version, this test will pass; in the vulnerable version, the test will fail.
#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test_error_propagation() {
        // Trigger exceptional condition with zero input.
        let result = run(0);
        assert_eq!(result, Err("Zero size"), "Expected error 'Zero size' when input is zero");
    }
}