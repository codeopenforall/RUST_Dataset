#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operation() {
        let engine = Engine::new();
        // For the given triggering input (150), the external_process returns 1 indicating an error.
        // The fixed behavior should return false (indicating failure), while the vulnerable version incorrectly returns true.
        let outcome = engine.process(150);
        assert_eq!(outcome, false, "Expected operation to fail for input > 100");
    }
}