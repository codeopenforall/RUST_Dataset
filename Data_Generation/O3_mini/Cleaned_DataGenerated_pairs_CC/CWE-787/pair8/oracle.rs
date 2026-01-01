#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_contract() {
        let mut manager = BufferManager::new();
        let original = manager.sum();
        let res = manager.update(5, 42);
        assert!(res.is_err(), "Expected error for out-of-bound update");
        assert_eq!(manager.sum(), original, "Buffer sum should remain unchanged on error");
    }
}
