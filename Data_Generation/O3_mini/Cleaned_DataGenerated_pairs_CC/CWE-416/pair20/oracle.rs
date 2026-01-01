#[cfg(test)]
mod test_oracle {
    use super::run;
    #[test]
    fn test_run_value() {
        let result = run();
        assert_eq!(result, 123, "Test failed: Expected 123, got {}", result);
    }
}
