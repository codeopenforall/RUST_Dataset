#[cfg(test)]
mod tests {
    use std::panic;

    // This test calls the execute function and asserts that no panic occurs.
    // In the vulnerable version the double free panic will be triggered causing the test to fail.
    // In the corrected version the function completes successfully and the test passes.
    #[test]
    fn test_resource_safety() {
        let result = panic::catch_unwind(|| {
            crate::execute();
        });
        assert!(
            result.is_ok(),
            "Test failed: execution panicked due to a double free vulnerability"
        );
    }
}