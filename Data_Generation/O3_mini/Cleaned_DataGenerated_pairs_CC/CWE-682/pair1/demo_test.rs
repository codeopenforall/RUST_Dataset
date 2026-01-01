#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calculation() {
        // For a Calculation with value 10 and factor 5, the correct result should be (10+5)/2 = 7.
        // The vulnerable version returns 12 whereas the fixed version returns 7.
        let calc = Calculation { value: 10 };
        let result = calc.compute(5);
        assert_eq!(result, 7, "Expected result to be 7, but got {}", result);
    }
}