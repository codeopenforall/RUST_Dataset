#[cfg(test)]
mod tests {
    use std::panic;

    // The test oracle calls the public API run_calc with inputs that lead to division by zero.
    // For the fixed version, run_calc is expected to return an Err with the message "Division by zero error".
    // For the vulnerable version, the underlying unsafe computation will panic (even though run_calc catches
    // the panic internally, its behavior diverges from the fixed contract). This test ensures that the correct
    // error case is handled without a panic.
    #[test]
    fn test_division_by_zero() {
        // The catch_unwind here ensures that if an unexpected panic occurs, the test will fail.
        let outcome = panic::catch_unwind(|| {
            crate::run_calc(10, 0)
        });
        match outcome {
            Err(_) => panic!("Unexpected panic occurred, indicating a vulnerability."),
            Ok(result) => {
                match result {
                    Ok(_) => panic!("Expected an error due to division by zero, but got a successful result."),
                    Err(msg) => assert_eq!(msg, "Division by zero error"),
                }
            }
        }
    }
}