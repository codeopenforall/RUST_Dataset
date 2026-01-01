#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_resource_limit_enforcement() {
        // Using a value above the maximum limit should produce an error.
        let count = 150;
        let handle = spawn_allocation(count);
        let result = handle.join().expect("Thread panicked in test");
        // The fixed version enforces the limit and must return an error.
        assert!(result.is_err(), "Expected error when exceeding resource limit");
    }
}