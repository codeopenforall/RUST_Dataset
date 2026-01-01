#[cfg(test)]
mod tests {
    use crate::calc;
    #[test]
    fn test_arithmetic_overflow() {
        let a: u64 = 3_000_000_000; 
        let b: u64 = 4;
        let result = calc::compute(a, b);
        assert!(result.is_err(), "Expected an error due to arithmetic overflow, but got Ok({:?})", result);
    }
}
