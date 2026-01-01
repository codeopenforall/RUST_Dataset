#[cfg(test)]
mod tests {
    use super::Calculator;
    #[test]
    fn test_overflow_detection() {
        let calc = Calculator {};
        let result = calc.compute(100_000, 100_000);
        assert!(result.is_err(), "Expected an overflow error, but got {:?}", result);
    }
}
