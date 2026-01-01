#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_oracle() {
        // The contract is that the compute() function must return exactly 100.
        // In the vulnerable version, the use-after-free may lead to an incorrect value.
        // In the fixed version, the function returns the expected value.
        assert_eq!(compute(), 100, "The computed result must be 100");
    }
}