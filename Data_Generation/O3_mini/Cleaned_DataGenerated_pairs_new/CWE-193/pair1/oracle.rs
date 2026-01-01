#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_accumulation() {
        let aggregator = Accumulator::new(10);
        aggregator.process();
        let result = aggregator.total();
        assert_eq!(result, 45, "Expected the sum to be 45, but got {}", result);
    }
}
