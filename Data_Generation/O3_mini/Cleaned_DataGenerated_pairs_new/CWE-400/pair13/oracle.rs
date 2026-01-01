#[cfg(test)]
mod tests {
    use super::run_service;
    #[test]
    fn test_resource_limit() {
        let input = 200;
        let result = run_service(input);
        assert_eq!(result, Err("limit exceeded"), "Expected error for input exceeding the limit");
    }
}
