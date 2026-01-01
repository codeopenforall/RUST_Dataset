#[cfg(test)]
mod tests {
    use std::panic;
    
    // This test oracle is designed to be run against both the vulnerable and corrected implementations.
    // It provides a deeply nested input that exceeds the safe recursion limit.
    // For the vulnerable version, the uncontrolled recursion is expected to cause a panic (stack overflow),
    // whereas the fixed version should gracefully return an error ("Recursion too deep").
    #[test]
    fn oracle() {
        // Create a deeply nested input: 1100 pairs of parentheses.
        let nested = "(".repeat(1100) + &")".repeat(1100);
        
        // Catch unwinding to differentiate between a panic and a returned error.
        let result = panic::catch_unwind(|| {
            // Unsafely call the execute API provided by the parser.
            // This function should either return a Result or panic.
            unsafe { super::Parser::execute(&nested) }
        });
        
        match result {
            Ok(res) => {
                // In the fixed version, the recursion depth check should trigger.
                match res {
                    Ok(_) => panic!("Expected failure due to deep recursion, but parsing succeeded."),
                    Err(e) => {
                        assert_eq!(e, "Recursion too deep", "Expected recursion depth limit error.");
                    }
                }
            },
            Err(_) => panic!("Function panicked due to uncontrolled recursion."),
        }
    }
}