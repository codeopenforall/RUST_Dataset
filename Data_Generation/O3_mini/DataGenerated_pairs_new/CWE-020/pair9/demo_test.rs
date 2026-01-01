#[cfg(test)]
mod tests {
    use super::*;
    
    // A helper function to simulate the processing using a given length.
    fn run_test_with_processor(process_fn: fn(&Engine, &str) -> Result<String, Box<dyn std::error::Error>>) {
        let engine = Engine::new("example");
        // "example" has 7 bytes. Requesting more should fail.
        let result = process_fn(&engine, "10");
        assert!(result.is_err(), "Should reject length greater than actual data size");
    }
    
    #[test]
    fn test_insecure_variant() {
        // For the insecure version, running with excessive length causes undefined behavior.
        // Here we assume the vulnerability will trigger an error or panic.
        // Since the behavior may be undefined, we catch the error in the unsafe branch.
        run_test_with_processor(Engine::execute);
    }
    
    #[test]
    fn test_secure_variant() {
        // For the secure version, the check prevents processing invalid input.
        // Using the same test should produce an error.
        run_test_with_processor(Engine::execute);
    }
}