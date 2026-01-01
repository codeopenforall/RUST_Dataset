#[cfg(test)]
mod tests {
    use super::Calculation;
    fn execute_calculation(calculator: &mut Calculation) {
        unsafe {
            calculator.calculate();
        }
    }
    #[test]
    fn percentage_test() {
        let mut calc = Calculation::new(200, 250);
        execute_calculation(&mut calc);
        assert_eq!(calc.percent, 25, "Expected 25% increase, but got a different value.");
    }
}
