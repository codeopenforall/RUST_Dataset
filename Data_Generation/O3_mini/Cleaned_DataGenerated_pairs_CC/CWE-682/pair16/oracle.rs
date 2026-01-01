#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_metric_calculation() {
        let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let aggregator = Aggregator::new(values);
        let result = aggregator.calculate();
        assert_eq!(result, 550, "Metric calculation failed: expected 550, got {}", result);
    }
}
