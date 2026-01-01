#[cfg(test)]
mod tests {
    use super::run_tasks;

    #[test]
    fn test_resource_limit() {
        // This input exceeds the allowed threshold.
        let high_input = 150;
        match run_tasks(high_input) {
            // The vulnerable version would process this without error,
            // but the fixed version should return an error.
            Ok(_) => panic!("Test failed: Resource limit was not enforced"),
            Err(e) => assert_eq!(e, "Task limit exceeded"),
        }
    }

    #[test]
    fn test_normal_operation() {
        let safe_input = 50;
        // Both versions should produce a valid sum if input is within limits.
        let result = run_tasks(safe_input);
        assert!(result.is_ok());
        // Compute expected sum = 0 + 1 + ... + (n-1)
        let expected: usize = (0..safe_input).sum();
        assert_eq!(result.unwrap(), expected);
    }
}