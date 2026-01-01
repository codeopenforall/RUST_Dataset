//////////////////////////////////////////////////////////////
// Test Oracle Function
// This test should be compiled separately from the vulnerable
// and fixed executables. It calls the process_update() function
// and asserts that the outcome is false, which is the expected
// result (operation failure) when an invalid index is provided.
// In the vulnerable code, the function erroneously returns true,
// causing the test to fail.
//////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    // Adjust the module path as necessary to refer to the function.
    use super::process_update;

    #[test]
    fn validate_update_outcome() {
        // For a request to update an out-of-bound index, the correct behavior is to fail.
        // This test will fail in the vulnerable code (which incorrectly reports success)
        // and pass in the fixed code.
        assert_eq!(
            process_update(), 
            false, 
            "Expected the out-of-bound update to fail, but it reported success."
        );
    }
}