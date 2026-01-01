//////////////////////////
// Test Oracle Function
//////////////////////////
#[cfg(test)]
mod tests {
    // Import the module under test. For the vulnerable build, this module will exhibit unsafe behavior.
    // For the fixed build, an out-of-bound operation returns an error.
    use super::process_operation;

    #[test]
    fn test_memory_bounds() {
        // Trigger the operation with an out-of-bound index.
        let op_result = process_operation(10);
        // The correct (fixed) behavior is to return an error when accessing an invalid index.
        // If the implementation is vulnerable, it will return Ok(99) (thus failing the test).
        assert!(
            op_result.is_err(),
            "Test failed: Operation should not succeed with an out-of-bound index."
        );
    }
}