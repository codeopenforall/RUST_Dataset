#[cfg(test)]
mod tests {
    use super::*;
    fn test_oracle() {
        let result = compute();
        assert_eq!(result, 42, "Unexpected result detected (use-after-free vulnerability).");
    }
    #[test]
    fn oracle_test() {
        test_oracle();
    }
}
