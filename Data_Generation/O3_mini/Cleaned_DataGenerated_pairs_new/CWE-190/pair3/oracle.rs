#[cfg(test)]
mod tests {
    use super::execute;
    #[test]
    fn test_total_computation() {
        let expected: u64 = 5_030_000_000;
        let result = execute();
        assert_eq!(result, expected, "Total does not match the expected value, indicating an integer overflow issue.");
    }
}
