#[cfg(test)]
mod tests {
    // Import the run_resource function from the module under test.
    use super::run_resource;

    #[test]
    fn test_resource_initialization() {
        // In the secure implementation, the resource is fully initialized:
        // field1 = 42 and field2 = 10, so process_resource() returns 52 per thread.
        // With 4 threads, the total should be 4 * 52 = 208.
        let expected_total = 208;
        let result = run_resource();
        assert_eq!(result, expected_total, "Resource sum mismatch: configuration may be uninitialized.");
    }
}