#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overflow_detection() {
        // Using the same triggering input for overflow.
        let processor = Processor { factor: 70000 };
        let input = 70000;
        let result = processor.check(input);
        // In the safe implementation the multiplication must trigger an overflow error.
        assert!(result.is_err(), "Expected an overflow error, but got {:?}", result);
    }
}