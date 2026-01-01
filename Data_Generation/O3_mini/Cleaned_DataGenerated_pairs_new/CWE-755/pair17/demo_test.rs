/////////////////////////////////////////////////////////////////
// Test Oracle - This test should fail when run against the vulnerable
// implementation and pass when run against the corrected version.
/////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oracle() {
        // The input "bad" is known to cause a parsing failure.
        // EXPECTED BEHAVIOR:
        // - Corrected implementation should return an error.
        // - Vulnerable implementation swallows the error, returning Ok(0), which is incorrect.
        let result = process_input("bad".to_string());
        assert!(
            result.is_err(),
            "Expected processing to fail for bad input, but got: {:?}", result
        );
    }
}