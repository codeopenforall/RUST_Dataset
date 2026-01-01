///////////////////////////////
// Test Oracle Function
///////////////////////////////
#[cfg(test)]
mod tests {
    use super::run_alloc;
    
    // This test verifies that a request exceeding the fixed limit results in an error.
    // For the vulnerable version (without cap), the test would pass erroneously,
    // showing the absence of proper resource throttling.
    #[test]
    fn test_resource_limit() {
        // Using a value that exceeds the limit (MAX_ALLOWED is 10).
        // The fixed implementation should return an error.
        let result = run_alloc(11);
        assert!(
            result.is_err(),
            "Test failed: Expected an error due to exceeding resource cap"
        );
    }
}