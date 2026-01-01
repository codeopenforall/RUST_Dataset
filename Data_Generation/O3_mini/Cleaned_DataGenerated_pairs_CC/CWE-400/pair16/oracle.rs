#[cfg(test)]
mod tests {
    use super::process;
    #[test]
    fn test_oracle() {
        let result = process(1500);
        assert!(result.is_err(), "Expected an error when the limit exceeds safe threshold");
    }
}
