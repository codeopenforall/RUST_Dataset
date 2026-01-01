//////////////////////////////////////////////
// Test Oracle Function
// This test verifies that the allocation size limit is enforced.
// In the corrected version, a request above the limit produces an Err result.
// In the original version, such a request might succeed (or even panic).
//////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::ResourceManager;

    #[test]
    fn allocation_limit_enforcement() {
        let mgr = ResourceManager::new();
        // Request an allocation size that exceeds the permitted maximum.
        let excessive_allocation = 2_000_000;
        let result = mgr.process(excessive_allocation);
        // The test asserts that the result is an error.
        // The vulnerable version would not have enforced a limit and hence might succeed.
        assert!(result.is_err(), "Allocation size should be limited by the fix");
    }
}