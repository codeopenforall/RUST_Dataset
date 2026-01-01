/*
   Test Oracle:
   This oracle function is designed to validate the common API function `initialize` implemented in both code versions.
   
   For the vulnerable version:
   - Calling initialize("bad") returns Ok(0) because the invalid input is silently converted to 0.
   - The test asserts that a valid configuration must not default to 0, so it fails when 0 is returned.

   For the fixed version:
   - Calling initialize("bad") returns an Err, correctly propagating the parsing error.
   - The test interprets the error as the expected outcome and passes.
   
   This single test functions as a contract ensuring that invalid configuration input leads to proper error handling.
*/
#[cfg(test)]
mod tests {
    // Assume the crate name is "application". When integrating, adjust the import as needed.
    use super::initialize;

    #[test]
    fn test_oracle() {
        let result = initialize("bad");
        match result {
            Ok(val) => {
                // Vulnerable behavior: invalid input produces a default 0.
                // The test will fail if val is 0.
                assert_ne!(val, 0, "Invalid configuration input must not default to 0");
            },
            Err(_) => {
                // Fixed behavior: an error is properly propagated.
                assert!(true, "Proper error occurred for invalid configuration");
            }
        }
    }
}