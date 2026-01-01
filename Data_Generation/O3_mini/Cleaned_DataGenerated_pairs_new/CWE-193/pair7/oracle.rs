#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;
    #[test]
    fn test_buffer_sum() {
        let mut handler = BufferHandler::new();
        handler.load(&[1, 2, 3, 4]);
        let shared_handler = Arc::new(handler);
        let handler_clone = Arc::clone(&shared_handler);
        let worker = thread::spawn(move || handler_clone.calculate());
        let result = worker.join().expect("Worker thread panicked");
        assert_eq!(result, 10, "The computed sum did not match the expected value.");
    }
}
