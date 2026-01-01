#[cfg(test)]
mod tests {
    use super::execute;

    // Oracle function: for an input exceeding the allowed MAX_TASKS, the fixed version should return an error,
    // whereas the vulnerable version would (incorrectly) proceed and return Ok.
    #[test]
    fn test_resource_throttling() {
        // Use a value that exceeds the limit (MAX_TASKS is 100 in the fixed version)
        let result = execute(150);
        // The fixed version should error out. If it returns Ok, then it's vulnerable.
        assert!(result.is_err(), "Expected error due to too many tasks, but got Ok");
    }
}