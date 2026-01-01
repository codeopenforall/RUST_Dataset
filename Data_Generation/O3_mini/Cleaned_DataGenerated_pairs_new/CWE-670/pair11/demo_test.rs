#[cfg(test)]
mod tests {
    use super::*;

    // This test verifies that processing a positive value correctly transitions the state to Running.
    // The expectation is that the fixed implementation returns true, whereas the flawed one returns false.
    #[test]
    fn test_state_transition() {
        let processor = Processor::new();
        let result = processor.process(10);
        // In the correct version, a positive input should move the state from Init to Running.
        // Thus, the assertion expects a true outcome.
        assert_eq!(result, true, "Expected state transition to Running did not occur");
    }
}