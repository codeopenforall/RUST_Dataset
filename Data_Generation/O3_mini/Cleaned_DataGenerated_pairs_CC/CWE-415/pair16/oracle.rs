#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::Ordering;
    #[test]
    fn test_drop_count() {
        GLOBAL_DROP.store(0, Ordering::SeqCst);
        process_resource();
        let count = GLOBAL_DROP.load(Ordering::SeqCst);
        assert_eq!(count, 1, "Expected drop count of 1, but got {}", count);
    }
}
