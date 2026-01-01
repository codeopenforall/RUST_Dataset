#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_value() {
        let result = unsafe { compute() };
        assert_eq!(result, 42, "Test failed: expected 42 but got {}", result);
    }
}
