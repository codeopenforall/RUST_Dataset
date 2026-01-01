#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_oracle() {
        let result = run_app();
        assert_eq!(
            result as u64,
            5000000000,
            "Product computation does not match expected value"
        );
    }
}
