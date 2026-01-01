#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_state_transition() {
        let mut ctl = Controller::new();
        let result = ctl.update(true);
        assert!(result, "Expected update(true) to return success");
        assert_eq!(
            ctl.mode,
            Mode::Active,
            "Expected Controller to be in Active state after update(true)"
        );
    }
}
