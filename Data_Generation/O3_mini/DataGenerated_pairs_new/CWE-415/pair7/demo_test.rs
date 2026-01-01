#[cfg(test)]
mod tests {
    use std::panic;

    #[test]
    fn test_memory_management() {
        // The function "process" is expected to be memory safe.
        // In the flawed (vulnerable) version, a panic will occur due to double free.
        // In the corrected version, process() will execute without panic.
        let result = panic::catch_unwind(|| {
            crate::process();
        });
        assert!(
            result.is_ok(),
            "Memory management function panicked due to double free vulnerability"
        );
    }
}