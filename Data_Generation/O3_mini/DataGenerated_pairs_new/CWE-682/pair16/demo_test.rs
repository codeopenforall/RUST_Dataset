#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_metric_calculation() {
        // Test input: Sum of [1,2,3,...,10] is 55 with 10 elements.
        // Correct metric should be (55 * 100) / 10 = 550.
        let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let aggregator = Aggregator::new(values);
        let result = aggregator.calculate();
        assert_eq!(result, 550, "Metric calculation failed: expected 550, got {}", result);
    }
}