#[cfg(test)]
mod tests {
    use super::*;
    
    // This test ensures that an allocation request exceeding the threshold is correctly rejected.
    #[test]
    fn test_allocation_limit() {
        let processor = Processor;
        // Using a value that exceeds the allowed maximum.
        let result = processor.handle(101);
        assert!(result.is_err(), "Expected error when allocation request exceeds limit");
    }
}