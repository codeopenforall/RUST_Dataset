#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_limit_enforcement() {
        let handler = ResourceHandler;
        // Provide an input exceeding the allowed threshold.
        let result = handler.execute_tasks(500);
        // Fixed code should return an error, while the vulnerable version would attempt resource allocation.
        assert!(result.is_err(), "Expected error due to too many tasks requested");
    }
}