#[cfg(test)]
mod tests {
    use std::panic;
    #[test]
    fn oracle() {
        let nested = "(".repeat(1100) + &")".repeat(1100);
        let result = panic::catch_unwind(|| {
            unsafe { super::Parser::execute(&nested) }
        });
        match result {
            Ok(res) => {
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
