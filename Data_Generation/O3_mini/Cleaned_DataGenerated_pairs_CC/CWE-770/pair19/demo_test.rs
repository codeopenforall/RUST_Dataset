//////////////////////////
// Test Oracle Function
//////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_allocation_limit() {
        // Select a count beyond the safe threshold to trigger the protection in the fixed code.
        // For the fixed version, this should return an error.
        let trigger_count = 2000;
        let result = run_threads(trigger_count);
        assert!(
            result.is_err(),
            "Expected error when count exceeds the allowed threshold"
        );
    }
}