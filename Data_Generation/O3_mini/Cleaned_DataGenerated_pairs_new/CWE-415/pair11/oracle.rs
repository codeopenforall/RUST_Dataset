#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::Ordering;
    #[test]
    fn test_resource_handling() {
        GLOBAL_COUNT.store(0, Ordering::SeqCst);
        let result = process();
        assert!(result.is_ok(), "Resource handling failed due to double free vulnerability");
    }
}
