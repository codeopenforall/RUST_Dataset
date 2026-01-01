#[cfg(test)]
mod tests {
    use super::Processor;
    use std::panic;
    #[test]
    fn recursion_oracle() {
        let depth = 1100;
        let input = "(".repeat(depth) + &")".repeat(depth);
        let proc = Processor;
        let result = panic::catch_unwind(|| proc.process(&input));
        match result {
            Ok(Ok(err)) => {
                panic!("Expected an error, but got a depth value: {}", err);
            },
            Ok(Err(e)) => {
                assert_eq!(e, "Recursion limit reached", "Expected recursion limit error");
            },
            Err(_) => {
                panic!("Function panicked due to uncontrolled recursion");
            }
        }
    }
}
