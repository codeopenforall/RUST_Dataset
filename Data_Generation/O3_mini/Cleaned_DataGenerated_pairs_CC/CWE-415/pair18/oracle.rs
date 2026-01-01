#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_double_free_protection() {
        GLOBAL_FREE_COUNT.store(0, Ordering::SeqCst);
        run();
        let count = free_count();
        assert_eq!(count, 1, "Double free vulnerability detected: free_count = {}", count);
    }
}
