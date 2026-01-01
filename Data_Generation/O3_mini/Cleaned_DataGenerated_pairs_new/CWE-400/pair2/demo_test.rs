#[cfg(test)]
mod tests {
    // Import the run_app function from the module.
    use super::run_app;
    const MAX_ALLOWED: usize = 100000;
    
    #[test]
    fn regression_test() {
        let count = run_app();
        // For the corrected version, the total count must not exceed MAX_ALLOWED.
        // In the vulnerable version, the unsynchronized accesses might allow count to exceed this limit.
        assert!(count <= MAX_ALLOWED, "Task count {} exceeds allowed maximum {}", count, MAX_ALLOWED);
    }
}