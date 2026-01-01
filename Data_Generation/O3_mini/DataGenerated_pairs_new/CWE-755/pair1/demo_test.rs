#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_exception_condition() {
        // Using a triggering input with a negative value.
        let inputs = vec![-1, 5];
        let result = run_contract(&inputs);
        // The contract expected for correct behavior is to return an error on negative input.
        // Therefore, if the function returns Ok, it indicates that the exception condition was swallowed.
        assert!(result.is_err(), "Expected error for negative input, got Ok: {:?}", result);
    }
}

// A standalone oracle function that can be used for external testing.
fn oracle() {
    let inputs = vec![-1, 5];
    let result = run_contract(&inputs);
    if result.is_ok() {
        panic!("Test oracle failed: expected error for negative input.");
    }
}