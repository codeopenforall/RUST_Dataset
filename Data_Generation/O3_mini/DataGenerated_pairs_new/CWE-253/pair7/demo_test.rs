#[cfg(test)]
mod tests {
    use super::*;
    
    // The following oracle test is designed to ensure that the operation
    // returns an error when provided an input of zero.
    // In the vulnerable version, the operation erroneously returns Ok(0),
    // thus the assertion below will fail.
    // In the fixed version, the error is correctly propagated.
    #[test]
    fn test_division_by_zero() {
        // Testing the process function with an input that triggers the error.
        let result = process(0);
        assert!(result.is_err(), "Expected an error for input 0, but got {:?}", result);
    }
}