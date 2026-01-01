#[cfg(test)]
mod tests {
    use super::*;
    fn run_test_with_processor(process_fn: fn(&Engine, &str) -> Result<String, Box<dyn std::error::Error>>) {
        let engine = Engine::new("example");
        let result = process_fn(&engine, "10");
        assert!(result.is_err(), "Should reject length greater than actual data size");
    }
    #[test]
    fn test_insecure_variant() {
        run_test_with_processor(Engine::execute);
    }
    #[test]
    fn test_secure_variant() {
        run_test_with_processor(Engine::execute);
    }
}
