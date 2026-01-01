#[cfg(test)]
mod tests {
    use super::{ResourceImpl, ResourceManager};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_resource_limit() {
        let limit = 1000;
        let manager = ResourceImpl::new(limit);
        manager.run();
        // Allow a fixed duration for the manager to accept connections.
        thread::sleep(Duration::from_secs(1));
        let count = manager.get_count();
        // In the secure version, the count should not exceed the limit.
        // In the vulnerable version, the count will far surpass the limit.
        assert!(
            count <= limit,
            "Resource usage exceeded limit: {} > {}",
            count,
            limit
        );
    }
}