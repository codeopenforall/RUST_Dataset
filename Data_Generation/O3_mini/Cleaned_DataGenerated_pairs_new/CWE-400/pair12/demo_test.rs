#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;

    // The acceptable maximum number of processed messages in the fixed version.
    const MAX_ACCEPTABLE: usize = 100;

    #[test]
    fn test_resource_control() {
        // Create an instance of the processing component.
        let handler = crate::Handler::new();
        
        // Enqueue 150 messages that are moderately sized.
        for _ in 0..150 {
            handler.submit("Test".repeat(10));
        }
        // Allow sufficient time for the background thread to process the messages.
        thread::sleep(Duration::from_millis(100));
        let count = crate::access_count();
        // In the corrected design, the number of processed messages should not exceed MAX_ACCEPTABLE.
        assert!(
            count <= MAX_ACCEPTABLE,
            "Resource consumption exceeded acceptable limit: {}",
            count
        );
    }
}