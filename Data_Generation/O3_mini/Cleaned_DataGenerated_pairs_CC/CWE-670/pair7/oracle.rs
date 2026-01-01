#[cfg(test)]
mod test_oracle {
    use super::Controller;
    #[test]
    fn command_processing_oracle() {
        let ctrl = Controller::new();
        let result = ctrl.process("start");
        assert!(result, "Expected processing a valid command to succeed.");
    }
}
