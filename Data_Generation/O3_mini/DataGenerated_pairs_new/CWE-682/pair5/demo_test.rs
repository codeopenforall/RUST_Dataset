#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average() {
        // For the input [1, 2, 3, 4] the correct average with integer division should be 2.
        // In the vulnerable implementation, each element is divided before summing,
        // leading to an incorrect result (1/4 + 2/4 + 3/4 + 4/4 == 0+0+0+1 == 1).
        let calc = Calculator::new(vec![1, 2, 3, 4]);
        let result = calc.process();
        // The vulnerable version would produce 1, so this assertion would fail.
        // The fixed version correctly computes 10/4, which truncates to 2.
        assert_eq!(result, 2, "Average calculation did not match expected value.");
    }
}