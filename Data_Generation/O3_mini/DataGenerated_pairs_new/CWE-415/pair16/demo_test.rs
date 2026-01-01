#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::Ordering;

    // This test resets the global counter, invokes the process, and asserts the drop count.
    // For the vulnerable version this test will fail (drop count becomes 2),
    // while for the corrected version the drop count will be exactly 1.
    #[test]
    fn test_drop_count() {
        GLOBAL_DROP.store(0, Ordering::SeqCst);
        process_resource();
        let count = GLOBAL_DROP.load(Ordering::SeqCst);
        assert_eq!(count, 1, "Expected drop count of 1, but got {}", count);
    }
}