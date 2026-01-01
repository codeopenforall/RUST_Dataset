/*
This test oracle is designed to verify the behavior of the parsing API.
It supplies an input with a deep nesting level.
For the original (vulnerable) version, the extremely deep input triggers a stack overflow (panic),
causing the test to fail. For the corrected version, the recursion depth check prevents an overflow
and instead returns an error, so the test passes.

Note: This test oracle function is separate from the main code. It uses catch_unwind
to detect panics in the vulnerable code.
*/
#[cfg(test)]
mod tests {
    use super::parse_nested;
    use std::panic;

    #[test]
    fn test_recursion_limit() {
        let deep_input = "(".repeat(1500) + &")".repeat(1500);
        let result = panic::catch_unwind(|| {
            parse_nested(&deep_input)
        });
        match result {
            // For the fixed version, parse_nested returns a Result without panicking.
            Ok(inner) => {
                // Expect an error from excessive recursion.
                // Using a generic match since error type differs between versions.
                if let Ok(_) = inner {
                    panic!("Expected error due to recursion depth, but got a valid AST");
                }
            }
            // For the vulnerable version, a panic occurs (e.g. stack overflow) causing the test to fail.
            Err(_) => {
                panic!("Test failed: uncontrolled recursion caused a panic (stack overflow)");
            }
        }
    }
}