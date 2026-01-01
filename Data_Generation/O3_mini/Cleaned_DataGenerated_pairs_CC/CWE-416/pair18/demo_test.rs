#[cfg(test)]
mod tests {
    use super::*;

    // This function serves as the test oracle.
    // When linked with the vulnerable version, it is expected to yield a result different from 42,
    // triggering the assertion failure (indicative of a use-after-free error).
    // With the fixed version, compute() correctly returns 42.
    fn test_oracle() {
        let result = compute();
        assert_eq!(result, 42, "Unexpected result detected (use-after-free vulnerability).");
    }

    #[test]
    fn oracle_test() {
        test_oracle();
    }
}