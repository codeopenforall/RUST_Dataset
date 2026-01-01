#[cfg(test)]
mod tests {
    use super::process_data;

    // Oracle test that fails for the vulnerable version (sum becomes 109) and passes for the fixed version (sum equals 10).
    #[test]
    fn test_memory_bounds() {
        let input = vec![1, 2, 3, 4];
        let expected_sum = 10;
        let result = process_data(&input);
        assert_eq!(result, expected_sum, "The computed sum should equal the correct sum without out-of-bound access.");
    }
}