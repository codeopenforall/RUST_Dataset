#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calculation() {
        let calc = Calc { rate: 5 };
        let result = calc.compute(200);
        assert_eq!(result, 10, "Expected deduction of 10 for revenue 200 with a 5% rate");
    }
}
