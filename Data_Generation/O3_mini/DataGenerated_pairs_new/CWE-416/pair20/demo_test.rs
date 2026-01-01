////////////////////////////////////////////
// Test Oracle: Unit Test for the "run" Function
////////////////////////////////////////////

#[cfg(test)]
mod test_oracle {
    // Import the run function from the module under test.
    // This test is designed to verify that the retrieved value is correct.
    // When linked against the vulnerable version, unpredictable behavior (or a crash)
    // should cause the test to fail. In the fixed version, the test will pass.
    use super::run;

    #[test]
    fn test_run_value() {
        let result = run();
        // The contract is that run() should reliably return 123.
        assert_eq!(result, 123, "Test failed: Expected 123, got {}", result);
    }
}