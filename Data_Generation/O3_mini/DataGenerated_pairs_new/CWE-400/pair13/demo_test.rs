#[cfg(test)]
mod tests {
    use super::run_service;

    #[test]
    fn test_resource_limit() {
        // The input value exceeds the safe threshold.
        let input = 200;
        let result = run_service(input);
        // For the fixed implementation, the function should return an error.
        // In the vulnerable code this test would incorrectly return Ok(_), causing the test to fail.
        assert_eq!(result, Err("limit exceeded"), "Expected error for input exceeding the limit");
    }
}