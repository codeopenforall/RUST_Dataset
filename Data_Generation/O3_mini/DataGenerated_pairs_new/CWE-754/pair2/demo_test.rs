// Test Oracle Function for CWE-754
// This unit test is designed to fail against the vulnerable code and pass with the fixed code.
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_exception() {
        let shared = Arc::new(Mutex::new(Shared::new(10)));
        // When the error flag is set by the spawned thread, perform_task() should return an error.
        let result = perform_task(shared, 5);
        assert!(result.is_err(), "Expected an error due to the exception flag, but got success");
    }
}