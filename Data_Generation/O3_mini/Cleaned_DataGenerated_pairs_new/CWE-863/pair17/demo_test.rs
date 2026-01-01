/////////////////////////////////////////////////////////////
// Test Oracle Function:
// This unit test verifies that when a triggering input above the allowed
// threshold (e.g., 2000) is provided, the execution returns an error.
// In the vulnerable version, execution would (likely) succeed (or hang) for
// high values, causing the test to fail; whereas in the corrected version the test
// passes by properly rejecting the input.
/////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn contract_test() {
        // Set a triggering input above the safe threshold.
        let trigger = 2000;
        let outcome = execute(trigger);
        // The test expects an error when the input exceeds allowed limit.
        assert!(outcome.is_err(), "Expected rejection for input: {}", trigger);
    }
}