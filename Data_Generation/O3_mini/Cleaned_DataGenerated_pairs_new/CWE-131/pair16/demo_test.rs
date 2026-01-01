///////////////////////////////////////////////////////////////
// Test Oracle Function
///////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::Data;

    #[test]
    fn test_data_copy() {
        let input = "This is a somewhat long input string causing miscalculation.";
        let data = Data::new(input);
        // The vulnerable version will only copy a truncated portion (e.g., 16 bytes)
        // whereas the fixed version should copy the entire string. 
        // If the output is not equal to the input, then the test fails.
        assert_eq!(data.as_str(), input, "Buffer content does not match input string");
    }
}