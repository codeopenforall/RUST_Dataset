#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;
    const MAX_ACCEPTABLE: usize = 100;
    #[test]
    fn test_resource_control() {
        let handler = crate::Handler::new();
        for _ in 0..150 {
            handler.submit("Test".repeat(10));
        }
        thread::sleep(Duration::from_millis(100));
        let count = crate::access_count();
        assert!(
            count <= MAX_ACCEPTABLE,
            "Resource consumption exceeded acceptable limit: {}",
            count
        );
    }
}
