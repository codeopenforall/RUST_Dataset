/* Test oracle function for verifying the behavior.
   This test calls the execute() method using an index (2) that corresponds to a buffer value of 3.
   - In the vulnerable version, this will trigger a panic due to the assertion, causing the test to fail.
   - In the corrected version, the input is gracefully handled without panicking, causing the test to pass.
   The test is deterministic and does not rely on timing assumptions.
*/
#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn test_oracle() {
        let processor = Processor::new();
        // For the vulnerable version, calling execute(2) will panic.
        // Our oracle expects a successful run (i.e. no panic) for the corrected version.
        let result = panic::catch_unwind(|| {
            processor.execute(2);
        });
        // The test fails (i.e. returns Err) in the vulnerable case, and passes (i.e. returns Ok) in the corrected case.
        assert!(result.is_ok(), "The process should not panic for index 2 in the corrected version");
    }
}