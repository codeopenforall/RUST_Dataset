#[cfg(test)]
mod tests {
    use std::panic;

    // Note: This test oracle is designed to run in an environment where either the vulnerable or the fixed code is being tested.
    // It verifies that for a deeply nested input string, the vulnerable version fails (typically with a panic or stack overflow)
    // while the fixed version returns a controlled error indicating that the maximum recursion depth has been exceeded.
    // The oracle uses panic::catch_unwind to capture potential panics in the vulnerable version.

    // Modify the following import to bring the `process` function from your code under test.
    use super::process;

    #[test]
    fn recursion_test() {
        // Create a deeply nested string: 150 opening and 150 closing parentheses.
        let input = "(".repeat(150) + &")".repeat(150);

        // Use catch_unwind to handle potential panics (stack overflow) in the vulnerable version.
        let result = panic::catch_unwind(|| process(&input));

        match result {
            // In the vulnerable version, a panic is likely to occur.
            Err(_) => {
                // Test failure condition: the vulnerable version should not successfully panic.
                panic!("Test failed: uncontrolled recursion led to a panic (stack overflow) in the vulnerable version.");
            },
            Ok(res) => {
                // For the fixed version, we expect a controlled error.
                match res {
                    Ok(_) => panic!("Test failed: input should not be processed successfully."),
                    Err(e) => {
                        assert_eq!(e, "Maximum recursion depth exceeded", "Test failed: unexpected error message.");
                    }
                }
            }
        }
    }
}