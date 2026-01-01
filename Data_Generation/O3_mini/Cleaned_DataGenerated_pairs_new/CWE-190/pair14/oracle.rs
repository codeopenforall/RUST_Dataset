#[cfg(test)]
mod tests {
    use super::run_and_fetch;
    #[test]
    fn test_counter() {
        let result = run_and_fetch();
        let expected: u32 = 0xFFFFFFFF;
        assert_eq!(result, expected, "The computed value should be clamped to u32::MAX");
    }
}
