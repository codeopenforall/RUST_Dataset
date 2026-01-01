#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use super::{DataProcessor, Average};

    #[test]
    fn test_exception_handling() {
        // Create a DataProcessor with an empty vector to trigger exceptional conditions.
        let processor = DataProcessor { data: Arc::new(Mutex::new(vec![])) };

        // For a correct implementation, an error should be returned for an empty dataset.
        let result = processor.average();

        match result {
            // In the vulnerable implementation, a default value (0) is returned, which is incorrect.
            Ok(val) => panic!("Test failed: Expected error, but got Ok({})", val),
            Err(msg) => assert_eq!(msg, "Empty dataset", "Unexpected error message"),
        }
    }
}