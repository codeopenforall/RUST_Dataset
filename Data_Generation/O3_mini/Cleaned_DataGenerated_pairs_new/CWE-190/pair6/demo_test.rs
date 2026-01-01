--------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    // This test oracle function uses a known large input value.
    // For the safe (fixed) version, the conversion substitutes i32::MAX.
    // The saturating multiplication: i32::MAX.saturating_mul(2) remains i32::MAX.
    // Then base (10) saturating added with i32::MAX still yields i32::MAX.
    // Thus, the expected result is i32::MAX.
    // In the unsafe version, the conversion yields -1, and the result becomes 8.
    #[test]
    fn test_oracle() {
        let calc = Calculator::new();
        let base = 10;
        let input: u32 = 0xFFFF_FFFF;
        let result = calc.compute(base, input);
        assert_eq!(result, i32::MAX, "Safe arithmetic result should saturate to i32::MAX");
    }
}
--------------------------------------------------