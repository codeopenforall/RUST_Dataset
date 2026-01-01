#[cfg(test)]
mod tests {
    use super::MemoryHandler;
    #[test]
    fn test_update_bounds() {
        let mut mgr = MemoryHandler::new();
        assert_eq!(mgr.update(50, 99), true, "Valid index update should succeed");
        assert_eq!(mgr.update(150, 99), false, "Out-of-bound update should fail");
    }
}
