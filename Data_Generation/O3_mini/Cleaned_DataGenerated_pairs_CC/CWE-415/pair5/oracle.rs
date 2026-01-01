#[cfg(test)]
mod tests {
    use std::panic;
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
