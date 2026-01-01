//////////////////////////////////////
// Test Oracle for Format String Issue
//////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    // This test verifies that the function properly handles a malicious format string.
    // In the vulnerable implementation, passing a malicious format string will lead to undefined behavior,
    // e.g., a panic or a crash. In the fixed implementation, the formatting is safely decoupled.
    #[test]
    fn test_format_safety() {
        let malicious = "%s%s%s";
        
        // Use catch_unwind to observe any panic (indicative of a vulnerability exploit).
        let result = panic::catch_unwind(|| {
            handle_message(malicious.to_string()).unwrap();
        });
        // The test passes only if no panic occurs.
        assert!(result.is_ok(), "Unexpected panic due to format string vulnerability");
    }
}