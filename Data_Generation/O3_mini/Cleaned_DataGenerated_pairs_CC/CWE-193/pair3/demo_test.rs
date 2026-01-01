#[cfg(test)]
mod tests {
    use super::*;

    // This test verifies that the sum computed is exactly 15.
    // In the vulnerable implementation, the out-of-bounds read would also include the extra field (value 99),
    // resulting in an incorrect sum (1+2+3+4+5+99 = 114). The fixed version should return 15.
    #[test]
    fn test_total_computation() {
        let holder = Holder::new();
        let result = compute_total(&holder);
        // Expected sum: 1 + 2 + 3 + 4 + 5 = 15
        assert_eq!(result, 15, "The total sum should be 15 but got {}", result);
    }
}