#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calculation() {
        let calc = Calculation { value: 10 };
        let result = calc.compute(5);
        assert_eq!(result, 7, "Expected result to be 7, but got {}", result);
    }
}
