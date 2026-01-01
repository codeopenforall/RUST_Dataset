#[cfg(test)]
mod tests {
    use super::compute;
    #[test]
    fn test_oracle() {
        let result = compute();
        assert_eq!(result, 42, "Test failed: expected 42, got {}", result);
    }
}
