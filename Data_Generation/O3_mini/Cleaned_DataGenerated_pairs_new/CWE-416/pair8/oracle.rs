#[cfg(test)]
mod tests {
    use super::compute_value;
    #[test]
    fn test_oracle() {
        let result = compute_value();
        assert_eq!(result, 123, "Test Failed: Expected 123, got {}", result);
    }
}
