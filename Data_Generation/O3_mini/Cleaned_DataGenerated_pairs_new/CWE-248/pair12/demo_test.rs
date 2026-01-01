#[cfg(test)]
mod tests {
    use std::panic;
    // Define a trait to uniformly query the error state of the output.
    trait Outcome {
        fn is_error(self) -> bool;
    }
    // For the fixed module, execute_task returns Result<i32, String>.
    impl Outcome for Result<i32, String> {
        fn is_error(self) -> bool {
            self.is_err()
        }
    }
    // For the vulnerable module, execute_task returns i32.
    impl Outcome for i32 {
        // In the vulnerable code, if the function returns a value, then no panic occurred.
        // However, as the negative input should have caused a panic, reaching here is an error.
        fn is_error(self) -> bool {
            false
        }
    }

    #[test]
    fn test_negative_input_handling() {
        // The oracle uses catch_unwind to intercept any unexpected panics.
        let outcome = panic::catch_unwind(|| crate::execute_task(-1));
        match outcome {
            // In the fixed version, the function call should not panic and should return an error result.
            Ok(result) => {
                assert!(result.is_error(), "Expected an error for negative input in safe handling");
            }
            // In the vulnerable version, a panic occurs, and the test will fail.
            Err(_) => panic!("Function panicked, which indicates unhandled exception vulnerability"),
        }
    }
}