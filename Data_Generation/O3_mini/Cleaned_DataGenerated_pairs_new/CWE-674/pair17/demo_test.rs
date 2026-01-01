//////////////////////////
// Test Oracle Start
//////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    // This function serves as the test oracle.
    // It feeds a deeply nested input to both versions.
    // The vulnerable version is expected to panic (or fail) due to stack overflow,
    // while the fixed version should return an error gracefully.
    #[test]
    fn test_recursive_limit() {
        // Create a deeply nested string.
        let deep_input = "(".repeat(1200) + &")".repeat(1200);

        // Test using the vulnerable process function:
        let vulnerable_result = panic::catch_unwind(|| {
            // This call is expected to lead to a stack overflow in the vulnerable version.
            process(&deep_input).unwrap()
        });
        // Assert that vulnerable version panicked or did not complete.
        assert!(vulnerable_result.is_err(), "Vulnerable version should not handle deep recursion gracefully");

        // Test using the fixed process function:
        // We simulate the fixed behavior by invoking the same function but assume the fix is applied.
        let fixed_result = process(&deep_input);
        match fixed_result {
            Err(e) => assert_eq!(e, "Recursion depth limit exceeded", "Fixed version should report recursion limit"),
            Ok(_) => panic!("Fixed version should not parse input beyond max depth"),
        }
    }
}
//////////////////////////
// Test Oracle End
//////////////////////////