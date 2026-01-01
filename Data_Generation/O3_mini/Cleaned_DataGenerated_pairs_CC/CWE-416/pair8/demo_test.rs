#[cfg(test)]
mod tests {
    use super::compute_value;

    #[test]
    fn test_oracle() {
        // The expected value is 123. In the vulnerable implementation, using the freed pointer
        // may return a different value (e.g. 456) due to memory reuse, thus failing the test.
        let result = compute_value();
        assert_eq!(result, 123, "Test Failed: Expected 123, got {}", result);
    }
}