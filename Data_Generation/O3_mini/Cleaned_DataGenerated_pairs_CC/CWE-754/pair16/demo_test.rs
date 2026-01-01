#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_exception_handling() {
        // Instantiate the worker.
        let worker = Worker { data: 10 };
        // Trigger the processing function which should detect the timeout exception.
        let result = worker.process();
        // For the vulnerable implementation this test would erroneously pass,
        // but for the fixed implementation it correctly errors.
        assert!(result.is_err(), "Expected error due to timeout condition");
        if let Err(msg) = result {
            assert_eq!(msg, "Timeout occurred");
        }
    }
}