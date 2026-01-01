#[cfg(test)]
mod tests {
    use super::compute;

    #[test]
    fn test_oracle() {
        // The expected result is 42. In the vulnerable implementation, the use-after-free
        // could lead to an incorrect value or a crash, whereas the corrected implementation
        // returns 42 reliably.
        let result = compute();
        assert_eq!(result, 42, "Test failed: expected 42, got {}", result);
    }
}