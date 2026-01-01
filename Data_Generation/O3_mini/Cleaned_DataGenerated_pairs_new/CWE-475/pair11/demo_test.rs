#[cfg(test)]
mod tests {
    use crate::run_harness;
    #[test]
    fn test_api_precondition() {
        // The contract is that when violating the API preconditions,
        // the fixed version detects the issue (returning an error)
        // while the vulnerable version erroneously returns an Ok result.
        let result = run_harness();
        assert!(
            result.is_err(),
            "Expected an error due to out-of-bound access, but got: {:?}",
            result
        );
    }
}