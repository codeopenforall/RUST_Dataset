#[cfg(test)]
mod tests {
    use std::panic;
    use std::sync::atomic::Ordering;
    use super::*;
    #[test]
    fn test_resource_deallocation() {
        FREED.store(0, Ordering::SeqCst);
        let resource = Resource::new(100);
        let result = panic::catch_unwind(|| {
            resource.release();
        });
        assert!(result.is_ok(), "Resource deallocation must not result in a double free panic");
    }
}
