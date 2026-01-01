#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_multiplication() {
        let input: u32 = 50_000;
        let expected: u64 = 50_000_u64 * 100_000;
        let result = simulate(input);
        assert_eq!(result, expected, "Integer multiplication did not produce the correct result, indicating a potential overflow vulnerability.");
    }
}
