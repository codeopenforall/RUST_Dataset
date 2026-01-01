#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;
    use std::sync::atomic::Ordering;
    #[test]
    fn test_resource_management() {
        GLOBAL_COUNT.store(0, Ordering::SeqCst);
        let result = panic::catch_unwind(|| {
            unsafe {
                let mut instance = Handler::create(123);
                instance.dispose();
            }
        });
        assert!(result.is_ok(), "Oracle: Expected resource to be safely managed (no double free)");
    }
}
