#[cfg(test)]
mod tests {
    use super::Processor;

    #[test]
    fn test_oracle() {
        // The input that in its correct handling should detect an overflow.
        let values = [9223372036854775808_u64, 4_u64];
        let result = Processor::calculate_product(&values);
        // The expected behavior is to detect an overflow and return an error.
        assert!(result.is_err(), "Expected an overflow error, but got a valid result.");
    }
}