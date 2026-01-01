#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_allocation_cap_enforcement() {
        let manager = ResourceManager::new();
        // This value exceeds the permitted limit of 1000.
        let result = manager.allocate(1500);
        // In the secure version, we expect an error to be returned.
        // In the vulnerable version, the error is not raised (and tasks are allocated).
        assert!(result.is_err(), "Allocation should fail when exceeding the safe limit");
        if let Err(msg) = result {
            assert_eq!(msg, "Requested tasks 1500 exceed limit of 1000");
        }
    }
}