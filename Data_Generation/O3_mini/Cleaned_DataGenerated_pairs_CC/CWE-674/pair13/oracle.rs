#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;
    #[test]
    fn test_excessive_nesting() {
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
