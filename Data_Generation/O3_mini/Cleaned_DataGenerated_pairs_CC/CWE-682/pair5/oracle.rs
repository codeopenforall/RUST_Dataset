#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_average() {
        let calc = Calculator::new(vec![1, 2, 3, 4]);
        let result = calc.process();
        assert_eq!(result, 2, "Average calculation did not match expected value.");
    }
}
