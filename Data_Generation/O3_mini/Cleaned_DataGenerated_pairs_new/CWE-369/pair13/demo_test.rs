#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn division_behavior() {
        // Test with a valid divisor; both versions should return correct result.
        let result = perform(5);
        assert_eq!(result.unwrap(), 20);

        // Test with a zero divisor.
        // In the fixed version, perform(0) returns an Err; in the vulnerable version, it triggers a panic.
        let result_zero = panic::catch_unwind(|| {
            perform(0)
        });
        match result_zero {
            Ok(res) => {
                // In fixed code, we expect an error message.
                assert!(res.is_err());
                assert_eq!(res.err().unwrap(), "Attempted division by zero");
            },
            Err(_) => {
                // In the vulnerable code, the panic indicates the vulnerability.
                panic!("Test failed: Division by zero caused a panic.");
            }
        }
    }
}