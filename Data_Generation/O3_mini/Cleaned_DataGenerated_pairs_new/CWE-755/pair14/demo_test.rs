//////////////////////////////////////////
// Test Oracle
// This test function serves as a single contract for both versions.
// When executing against the vulnerable version, the test will fail because
// the invalid input ("abc") causes a panic (or unintended behavior).
// When executing against the fixed version, the test passes by properly returning an error.
//////////////////////////////////////////

#[cfg(test)]
mod tests {
    use std::panic;
    // The API function 'perform' is expected to be present in both versions.
    use super::perform;

    #[test]
    fn test_oracle() {
        // Test with valid input: expect a successful computation.
        {
            let res = panic::catch_unwind(|| perform("42"));
            assert!(res.is_ok(), "Valid input caused a panic unexpectedly");
            let value = res.unwrap().unwrap();
            assert_eq!(value, 100 / 42, "Incorrect computation for valid input");
        }
        // Test with invalid input (non-numeric).
        // In the fixed version, this should return an error.
        // In the vulnerable version, it is likely to panic or produce unintended behavior.
        {
            let res = panic::catch_unwind(|| perform("abc"));
            if let Ok(result) = res {
                // For the fixed version, result should be Err.
                assert!(
                    result.is_err(),
                    "Expected error for non-numeric input, but got success: {:?}",
                    result.ok()
                );
            } else {
                panic!("Non-numeric input caused a panic, indicating the vulnerability.");
            }
        }
    }
}