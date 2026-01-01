/////////////////////// Test Oracle ///////////////////////
#[cfg(test)]
mod test_oracle {
    use super::Controller;

    // The oracle function tests that processing a valid "start" command returns true.
    // For the vulnerable version, this test would fail (returning false); for the fixed version, it passes.
    #[test]
    fn command_processing_oracle() {
        let ctrl = Controller::new();
        let result = ctrl.process("start");
        // The expected behavior is that with a valid command, the state should be updated properly.
        assert!(result, "Expected processing a valid command to succeed.");
    }
}