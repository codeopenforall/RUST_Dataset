#[cfg(test)]
mod tests {
    // Import the definitions from the module containing MemoryManager.
    use super::*;

    // The test verifies that processing a request exceeding the allowed limit fails.
    // In the vulnerable version, process(2000) would succeed (or cause resource issues),
    // while in the corrected version it returns an error.
    #[test]
    fn test_allocation_throttling() {
        let manager = MemoryManager{};
        let result = manager.process(2000);
        assert!(result.is_err(), "Expected error due to allocation size limit, but got success.");
    }
}