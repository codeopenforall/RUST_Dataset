#[cfg(test)]
mod tests {
    use std::panic::{catch_unwind, AssertUnwindSafe};
    use crate::Parser;
    #[test]
    fn oracle_test() {
        let deep_input = format!("{}{}", "(".repeat(1100), ")".repeat(1100));
        let result = catch_unwind(AssertUnwindSafe(|| {
            let mut parser = Parser::new(&deep_input);
            parser.parse_expr()
        }));
        match result {
            Ok(Err(e)) => {
                assert_eq!(e, "Maximum recursion depth exceeded", "Expected maximum depth error");
            },
            Ok(Ok(_node)) => panic!("Expected an error due to deep recursion, but parsing succeeded."),
            Err(_) => panic!("Parsing caused a panic (stack overflow) in the vulnerable version."),
        }
    }
}
