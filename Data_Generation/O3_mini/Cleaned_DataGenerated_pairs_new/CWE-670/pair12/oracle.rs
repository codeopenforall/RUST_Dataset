#[cfg(test)]
mod tests {
    use super::Engine;
    #[test]
    fn test_engine_safe_update() {
        let system = Engine::new();
        let outcome = system.process(1).expect("Expected a valid result");
        assert_eq!(outcome, 5, "For valid input, the state should be incremented by 5.");
    }
}
