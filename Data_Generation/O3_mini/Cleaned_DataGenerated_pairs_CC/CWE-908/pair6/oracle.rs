#[cfg(test)]
mod tests {
    use super::run_resource;
    #[test]
    fn test_resource_initialization() {
        let expected_total = 208;
        let result = run_resource();
        assert_eq!(result, expected_total, "Resource sum mismatch: configuration may be uninitialized.");
    }
}
