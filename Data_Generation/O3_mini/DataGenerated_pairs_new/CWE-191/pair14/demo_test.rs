//////////////////////////////////////////////////////////////
// Test Oracle: This unit test verifies that using an initial value
// of 100 and subtracting 200 should trigger an error condition in
// the corrected implementation, but not in the vulnerable one.
// The test is designed to fail for the vulnerable version and pass
// for the fixed version.
//////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    // Import the process_operation function from the module.
    use super::process_operation;

    #[test]
    fn test_underflow_behavior() {
        let initial = 100u32;
        let subtractor = 200u32;
        let result = process_operation(initial, subtractor);
        
        // In the fixed version, the subtraction should trigger an underflow error.
        // Thus, we expect the result to be an error.
        assert!(
            result.is_err(),
            "Expected error due to integer underflow, but got: {:?}",
            result.ok()
        );
    }
}