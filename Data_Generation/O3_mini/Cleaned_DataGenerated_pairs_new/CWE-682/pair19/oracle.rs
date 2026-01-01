#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn oracle_test() {
        let processor = TimeProcessor::new();
        let expected = 500_000;
        let actual = processor.compute(500);
        assert_eq!(actual, expected, "Conversion calculation is incorrect.");
    }
}
