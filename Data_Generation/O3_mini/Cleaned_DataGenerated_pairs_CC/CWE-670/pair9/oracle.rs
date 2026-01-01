#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_control_flow() {
        let dsp = Dispatcher::new(true);
        let test_input = 3;
        let outcome = dsp.process(test_input);
        assert_eq!(outcome, 8, "The operation branch is incorrectly selected based on untrusted input.");
    }
}
