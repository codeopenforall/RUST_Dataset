#[cfg(test)]
mod tests {
    use super::*;

    // The following test oracle verifies the correctness of the calculation.
    // For a revenue of 200 and a rate of 5, the correct deduction should be 10.
    // In the vulnerable version the result will be 0, causing the test to fail.
    #[test]
    fn test_calculation() {
        let calc = Calc { rate: 5 };
        let result = calc.compute(200);
        assert_eq!(result, 10, "Expected deduction of 10 for revenue 200 with a 5% rate");
    }
}