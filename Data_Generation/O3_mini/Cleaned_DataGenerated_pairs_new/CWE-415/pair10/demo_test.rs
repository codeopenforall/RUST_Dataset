#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;
    use std::sync::atomic::Ordering;

    // This test oracle function expects the safe implementation to execute
    // without causing a panic. For the vulnerable path, the double free will
    // trigger a panic, causing the test to fail.
    #[test]
    fn test_resource_management() {
        // Reset the global counter to ensure a clean test environment.
        GLOBAL_COUNT.store(0, Ordering::SeqCst);

        // The test will catch any panics during the resource's lifetime.
        let result = panic::catch_unwind(|| {
            unsafe {
                let mut instance = Handler::create(123);
                instance.dispose();
                // On drop, if the pointer is not null, deallocation is attempted.
            }
        });
        // Oracle expects no panic (i.e. no double free) for the corrected version.
        assert!(result.is_ok(), "Oracle: Expected resource to be safely managed (no double free)");
    }
}