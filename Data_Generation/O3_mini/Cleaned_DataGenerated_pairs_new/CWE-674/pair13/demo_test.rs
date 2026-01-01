#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    // This test constructs an extremely deeply nested input.
    // In the vulnerable version, uncontrolled recursion is expected to cause a stack overflow (panic).
    // In the fixed version, execute() should return an error indicating the recursion limit was exceeded.
    #[test]
    fn test_excessive_nesting() {
        // Build a deeply nested input: 1500 opening parentheses, a token, and 1500 closing parentheses.
        let mut input = String::new();
        for _ in 0..1500 {
            input.push('(');
        }
        input.push('a');
        for _ in 0..1500 {
            input.push(')');
        }
        
        // Use catch_unwind to safely capture a panic in case of a stack overflow.
        let result = panic::catch_unwind(|| execute(&input));
        
        match result {
            // If a panic is caught, then the vulnerable implementation likely overflowed.
            Err(_) => panic!("Test failed: Uncontrolled recursion triggered a panic (stack overflow)."),
            Ok(inner) => {
                // Expect the fixed version to return an error due to recursion depth limit.
                let outcome = inner;
                match outcome {
                    Ok(val) => panic!("Test failed: Expected an error due to recursion depth; returned value {}", val),
                    Err(e) => assert_eq!(e, "Recursion depth limit exceeded", "Unexpected error message"),
                }
            }
        }
    }
}