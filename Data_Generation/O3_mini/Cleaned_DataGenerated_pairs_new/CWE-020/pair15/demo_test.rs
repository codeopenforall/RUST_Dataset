///////////////////////////////////////////////
// Test Oracle Function for CWE-020 Example
///////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::Config;
    use std::panic;

    // A deterministic test input that is malformed: the header indicates a payload length
    // that exceeds the available data. In the secure version, this should be handled gracefully
    // with an error. In the vulnerable version, unsafe access may trigger a panic.
    const MALFORMED_INPUT: &str = "10short";

    #[test]
    fn test_input_validation() {
        // Catch panics to differentiate between panic and error return.
        let vulnerable_result = panic::catch_unwind(|| Config::parse(MALFORMED_INPUT));
        // For the secure version, the function should return an error, not panic.
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