#[cfg(test)]
mod tests {
    use std::panic;
    #[test]
    fn test_resource_double_free() {
        let result = panic::catch_unwind(|| {
            crate::main();
        });
        #[cfg(feature = "issue")]
        assert!(result.is_err(), "Expected panic due to double free");
        #[cfg(not(feature = "issue"))]
        assert!(result.is_ok(), "Expected successful run without panic");
    }
}
