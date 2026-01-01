#[cfg(test)]
mod tests {
    use std::panic;
    // We assume the presence of run_tasks in the tested module.
    // The test passes only if a panic occurs for excessive tasks.
    #[test]
    fn oracle_resource_limit() {
        // Here, 150 exceeds the maximum allowed task count.
        let result = panic::catch_unwind(|| {
            // This call uses the run_tasks from the current module.
            // For the fixed version, this should panic.
            super::run_tasks(150);
        });
        // Test passes only if a panic occurs, indicating proper limitation.
        assert!(result.is_err(), "Expected a panic due to excessive resource allocation");
    }
}