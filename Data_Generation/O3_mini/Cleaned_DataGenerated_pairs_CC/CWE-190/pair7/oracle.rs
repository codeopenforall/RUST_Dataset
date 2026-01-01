#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_overflow_protection() {
        let mut calc = Calculator { sum: u32::MAX - 5 };
        calc.add(10);
        calc.add(10);
        assert_eq!(calc.get(), u32::MAX);
    }
}
