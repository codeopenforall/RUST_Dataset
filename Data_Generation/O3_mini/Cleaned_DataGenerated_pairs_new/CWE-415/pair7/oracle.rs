#[cfg(test)]
mod tests {
    use std::panic;
    #[test]
    fn test_memory_management() {
        let result = panic::catch_unwind(|| {
            crate::process();
        });
        assert!(
            result.is_ok(),
            "Memory management function panicked due to double free vulnerability"
        );
    }
}
