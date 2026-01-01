#[cfg(test)]
mod tests {
    use super::MemoryHandler;

    // Test oracle function: For the fixed version, updating an out-of-bound index should return false.
    // For the vulnerable version, it would incorrectly return true and corrupt memory.
    #[test]
    fn test_update_bounds() {
        let mut mgr = MemoryHandler::new();
        // In-range update - should succeed.
        assert_eq!(mgr.update(50, 99), true, "Valid index update should succeed");
        // Out-of-bound update - should be rejected in the fixed version.
        assert_eq!(mgr.update(150, 99), false, "Out-of-bound update should fail");
    }
}