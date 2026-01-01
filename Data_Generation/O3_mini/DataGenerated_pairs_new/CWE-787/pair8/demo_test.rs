#[cfg(test)]
mod tests {
    // Bring the definition of BufferManager into test scope.
    use super::*;

    #[test]
    fn test_contract() {
        let mut manager = BufferManager::new();
        let original = manager.sum();
        let res = manager.update(5, 42);
        // In the safe implementation, an out-of-bound update should be rejected with an error.
        assert!(res.is_err(), "Expected error for out-of-bound update");
        // The buffer must remain unchanged.
        assert_eq!(manager.sum(), original, "Buffer sum should remain unchanged on error");
    }
}