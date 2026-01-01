#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::Ordering;
    #[test]
    fn test_double_free_detection() {
        FREE_COUNT.store(0, Ordering::SeqCst);
        unsafe {
            let mut res = Resource::new(42);
            res.release();
        }
        assert_eq!(
            FREE_COUNT.load(Ordering::SeqCst),
            1,
            "Double free detected: unexpected number of deallocations"
        );
    }
}
