#[cfg(test)]
mod tests {
    use super::*;
    // A test oracle that ensures the correct branch is executed.
    // In safe mode, the outcome must always be computed by nonsensitive(), i.e., input + 5.
    #[test]
    fn test_control_flow() {
        // Create an instance with safe_mode activated.
        let dsp = Dispatcher::new(true);
        // Test input chosen to trigger the flawed branch in the vulnerable version (odd number).
        let test_input = 3;
        let outcome = dsp.process(test_input);
        // Expected outcome: nonsensitive(3) = 3 + 5 = 8.
        // In the vulnerable version, outcome would be sensitive(3) = 30.
        assert_eq!(outcome, 8, "The operation branch is incorrectly selected based on untrusted input.");
    }
}