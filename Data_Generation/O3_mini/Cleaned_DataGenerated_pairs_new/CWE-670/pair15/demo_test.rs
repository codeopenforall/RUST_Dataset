//
// Test oracle function to verify the vulnerability fix.
// This function should be placed in a separate tests module and is not part of the main binaries.
//
// Note: The test uses a single-threaded call to simplify validation of the update and check functions.
// For the vulnerable version, the test will fail as the incorrect control flow returns a denied state.
// For the fixed version, the test passes.
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_control() {
        let handler = Handler::new();
        // Triggering input (150) should grant access.
        handler.update(150);
        assert!(handler.check(), "Access should be granted for input 150");
    }
}