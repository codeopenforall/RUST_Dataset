#[cfg(test)]
mod tests {
    use super::Config;
    use std::panic;
    const MALFORMED_INPUT: &str = "10short";
    #[test]
    fn test_input_validation() {
        let vulnerable_result = panic::catch_unwind(|| Config::parse(MALFORMED_INPUT));
        match vulnerable_result {
            Ok(res) => {
                assert!(
                    res.is_err(),
                    "Expected error due to malformed input, but got Ok result"
                );
            }
            Err(_) => {
                panic!("Function panicked on malformed input, which indicates vulnerability");
            }
        }
    }
}
