#[cfg(test)]
mod tests {
    use std::panic;
    use super::process;
    #[test]
    fn recursion_test() {
        let input = "(".repeat(150) + &")".repeat(150);
        let result = panic::catch_unwind(|| process(&input));
        match result {
            Err(_) => {
                panic!("Test failed: uncontrolled recursion led to a panic (stack overflow) in the vulnerable version.");
            },
            Ok(res) => {
                match res {
                    Ok(_) => panic!("Test failed: input should not be processed successfully."),
                    Err(e) => {
                        assert_eq!(e, "Maximum recursion depth exceeded", "Test failed: unexpected error message.");
                    }
                }
            }
        }
    }
}
