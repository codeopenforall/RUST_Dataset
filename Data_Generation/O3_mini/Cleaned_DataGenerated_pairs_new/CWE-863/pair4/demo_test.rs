////////////////////////////////////////////////////////////
// The following test oracle verifies that excessive task
// requests are properly rejected. When executed against the
// vulnerable implementation, the test fails (since no cap is set),
// but it passes with the fixed implementation where a limit is enforced.
////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_allocation_limit() {
        let service = ResourceService;
        // Request a number of tasks that exceeds the safe limit.
        let result = service.execute_request(1000);
        // The secure behavior is to return an error when the request exceeds the limit.
        assert!(
            result.is_err(),
            "Exceeding the safe limit should be rejected."
        );
    }
}