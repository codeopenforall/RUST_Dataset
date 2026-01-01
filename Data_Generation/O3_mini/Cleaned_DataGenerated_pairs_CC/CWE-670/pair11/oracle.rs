#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_state_transition() {
        let processor = Processor::new();
        let result = processor.process(10);
        assert_eq!(result, true, "Expected state transition to Running did not occur");
    }
}
