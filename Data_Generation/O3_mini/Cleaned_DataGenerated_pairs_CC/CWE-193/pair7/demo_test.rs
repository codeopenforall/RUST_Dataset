#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    // This oracle test is designed so that:
    // - In the vulnerable version, the off-by-one error will cause out-of-bound access
    //   and the test will panic or produce an incorrect sum.
    // - In the fixed version, the sum will be computed correctly as 10.
    #[test]
    fn test_buffer_sum() {
        let mut handler = BufferHandler::new();
        // Input array: expected sum is 1 + 2 + 3 + 4 = 10.
        handler.load(&[1, 2, 3, 4]);
        let shared_handler = Arc::new(handler);
        let handler_clone = Arc::clone(&shared_handler);
        let worker = thread::spawn(move || handler_clone.calculate());
        let result = worker.join().expect("Worker thread panicked");
        assert_eq!(result, 10, "The computed sum did not match the expected value.");
    }
}