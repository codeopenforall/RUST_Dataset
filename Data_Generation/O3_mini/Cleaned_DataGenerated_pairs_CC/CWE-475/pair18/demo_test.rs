#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    // This test uses an empty byte slice as triggering input.
    // For the vulnerable version, the unchecked pointer arithmetic should
    // lead to undefined behavior (and typically a panic in debug builds),
    // while the fixed version safely returns 0.
    #[test]
    fn test_empty_input_handling() {
        let input = b"";
        let calc = Calculator;
        let result = panic::catch_unwind(|| calc.run(input));
        match result {
            // In the fixed version, no panic occurs and the function returns 0.
            Ok(val) => {
                assert_eq!(val, 0, "Expected a 0 return value for empty input in the safe implementation");
            }
            // In the vulnerable build, the function is expected to panic due to undefined behavior.
            Err(_) => {
                panic!("Function panicked for empty input, indicating unsafe handling of API preconditions");
            }
        }
    }
}