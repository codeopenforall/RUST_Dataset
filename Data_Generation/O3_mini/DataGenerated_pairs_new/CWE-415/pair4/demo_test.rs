//////////////////////////////
// Test Oracle Function
// This test is expected to panic (or fail) when linked with the vulnerable version
// and to pass (return normally) when run against the corrected version.
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test_memory_release() {
        // The vulnerable version triggers a panic due to double free.
        // In the corrected version, run() should complete without panicking.
        // Since the behavior is deterministic, we wrap run() in catch_unwind.
        let result = std::panic::catch_unwind(|| {
            run();
        });
        assert!(result.is_ok(), "Resource deallocation failed due to double free");
    }
}