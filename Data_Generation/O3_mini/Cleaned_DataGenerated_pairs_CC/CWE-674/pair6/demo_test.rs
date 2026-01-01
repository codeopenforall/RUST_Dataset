//////////////////////////////////////////
// Test Oracle Function
// This test executes the process() function with an excessively deep input.
// It asserts that the result is an error with the message "Recursion limit exceeded".
// For the vulnerable version, the test is expected to fail (stack overflow or panic).
// For the fixed version, the error is properly returned and the test passes.
//////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_recursion_limit() {
        // Construct an input string with 1200 nested pairs.
        let nested_input = "(".repeat(1200) + &")".repeat(1200);
        let result = process(&nested_input);
        match result {
            Ok(_) => panic!("Expected error due to excess recursion depth"),
            Err(e) => assert_eq!(e, "Recursion limit exceeded"),
        }
    }
}