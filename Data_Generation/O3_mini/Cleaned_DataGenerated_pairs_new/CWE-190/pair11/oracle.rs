#[cfg(test)]
mod tests {
    use super::run_accumulation;
    #[test]
    fn test_arithmetic_correctness() {
        let iterations = 1000;
        let threads = 4;
        let a = 50000;
        let b = 100000;
        let expected: i64 = (threads as i64) 
            * (iterations as i64) 
            * ((a as i64).checked_mul(b as i64).unwrap());
        let result = run_accumulation(a, b, iterations, threads);
        assert_eq!(
            result, expected,
            "The computed total ({}) does not match the expected value ({}).",
            result, expected
        );
    }
}
