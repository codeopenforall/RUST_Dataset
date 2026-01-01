//////////////////////////////////////////
// Test Oracle for Recursion Vulnerability //
//////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    // This test uses an extremely nested input to trigger the recursion.
    // For the corrected version it should produce an error with a specific message,
    // while the vulnerable version will panic (stack overflow) and thus fail the test.
    #[test]
    fn test_recursion_limit() {
        // Create a deeply nested input designed to exceed MAX_DEPTH.
        let deep_input = "(".to_owned() + &"(".repeat(1500) + "x" + &")".repeat(1500) + ")";
        let result = panic::catch_unwind(|| execute(&deep_input));
        match result {
            Ok(Ok(_)) => panic!("Expected error due to recursion limit being exceeded, but got successful parse."),
            Ok(Err(e)) => assert_eq!(e, "recursion limit exceeded"),
            Err(_) => panic!("Function panicked, which reflects the vulnerability in the uncontrolled recursion case."),
        }
    }
}