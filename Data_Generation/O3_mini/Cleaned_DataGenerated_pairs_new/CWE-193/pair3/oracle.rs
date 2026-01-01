#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_total_computation() {
        let holder = Holder::new();
        let result = compute_total(&holder);
        assert_eq!(result, 15, "The total sum should be 15 but got {}", result);
    }
}
