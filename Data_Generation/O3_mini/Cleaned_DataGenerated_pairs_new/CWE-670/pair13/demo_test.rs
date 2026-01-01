#[cfg(test)]
mod tests {
    // Bring the items from the parent module into scope.
    use super::*;

    #[test]
    fn test_state_transition() {
        // Initialize the controller in Idle state.
        let mut ctl = Controller::new();
        // Trigger an update expecting a transition to Active.
        let result = ctl.update(true);

        // For the vulnerable version, result will be true but the state remains Idle.
        // For the fixed version, the state will correctly become Active.
        assert!(result, "Expected update(true) to return success");
        assert_eq!(
            ctl.mode,
            Mode::Active,
            "Expected Controller to be in Active state after update(true)"
        );
    }
}