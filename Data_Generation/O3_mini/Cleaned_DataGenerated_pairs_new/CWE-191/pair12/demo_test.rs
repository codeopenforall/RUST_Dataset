#[cfg(test)]
mod tests {
    // Import the public API function from the user module.
    use super::execute_subtraction;

    #[test]
    fn test_underflow_prevention() {
        // For safe (fixed) code, subtracting 10 from 5 must return an Err.
        // In the vulnerable code, the function would return Ok(wrapped_value).
        let result = execute_subtraction();
        assert!(result.is_err(),
                "Expected an error due to underflow prevention, but got {:?}", result);
    }
}