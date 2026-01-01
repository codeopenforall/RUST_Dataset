#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_recursion_limit() {
        let nested_input = "(".repeat(1200) + &")".repeat(1200);
        let result = process(&nested_input);
        match result {
            Ok(_) => panic!("Expected error due to excess recursion depth"),
            Err(e) => assert_eq!(e, "Recursion limit exceeded"),
        }
    }
}
