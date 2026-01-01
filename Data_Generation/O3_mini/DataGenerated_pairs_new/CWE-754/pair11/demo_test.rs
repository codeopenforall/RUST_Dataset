#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicI32, Ordering};

    // The test is designed such that the asynchronous task takes longer than the timeout.
    // Therefore, a correct implementation should return an error.
    #[test]
    fn test_timeout_condition() {
        let processor = Processor { value: Arc::new(AtomicI32::new(42)) };
        let outcome = processor.execute();
        
        // Assert that the outcome is an error.
        assert!(outcome.is_err(), "Expected error due to timeout, but got: {:?}", outcome);
        
        if let Err(msg) = outcome {
            assert_eq!(msg, "Operation timed out", "Error message mismatch");
        }
    }
}