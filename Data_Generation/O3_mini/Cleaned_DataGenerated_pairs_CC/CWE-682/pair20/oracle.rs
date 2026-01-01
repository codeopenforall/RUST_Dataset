#[cfg(test)]
mod tests {
    use super::Calculator;
    #[test]
    fn oracle_test() {
        let calc = Calculator { scale: 10 };
        unsafe {
            let computed = calc.compute(100, 5);
            assert_eq!(computed, 2, "Computation error: expected 2, got {}", computed);
        }
    }
}
