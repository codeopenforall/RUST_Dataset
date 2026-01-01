#[cfg(test)]
mod tests {
    use std::panic;
    use super::execute_app;

    #[test]
    fn test_edge_input() {
        // Test with an input value equal to the threshold.
        // In the vulnerable version, this input would trigger the assertion and panic (leading to DoS).
        // In the fixed version, the input is sanitized so that no panic occurs.
        let result = panic::catch_unwind(|| {
            execute_app(1000);
        });
        // The oracle expects the application NOT to panic.
        assert!(
            result.is_ok(),
            "Application panicked with edge input value, indicating a vulnerability"
        );
    }
}