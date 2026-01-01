#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exceptional_condition() {
        let worker = Worker::new();
        // For input 1 (an odd number), the process is designed to time out.
        // In the fixed version, execute returns an error.
        // In the vulnerable version, it erroneously returns a default value (100).
        let result = worker.execute(1);
        match result {
            Err(ProcError::Timeout) => assert!(true, "Proper error propagated"),
            Ok(val) => panic!("Test failed: Expected error, but got success with value: {}", val),
            _ => panic!("Test failed: Unexpected result variant"),
        }
    }
}