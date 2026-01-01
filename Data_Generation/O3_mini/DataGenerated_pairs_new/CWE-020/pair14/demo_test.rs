//////////////// Test Oracle ////////////////////
// This test oracle is designed to validate the contract of the function:
// When provided with malformed or malicious input (where the declared length is larger than the available data),
// the corrected (fixed) implementation should return an error,
// while the vulnerable version would panic or produce an incorrect result.
// This test is expected to fail when linked with the vulnerable implementation and pass with the corrected one.

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::Any;

    #[test]
    fn oracle_test() {
        // This input declares a length "0010" meaning 10 characters
        // but actually provides only 3 characters after the field.
        let malicious_input = "0010abc";
        let processor = DataProcessor;

        // Catch any panic that might occur.
        let result = std::panic::catch_unwind(|| {
            processor.process(malicious_input)
        });

        // The function should not panic.
        assert!(result.is_ok(), "Function panicked on invalid input");

        // Extract the value returned from the function.
        let output = result.unwrap();

        // Now, using dynamic type checking, differentiate between the vulnerable
        // version (which returns a plain String) and the fixed version (which returns a Result).
        if let Some(fixed_result) = (&output as &dyn Any).downcast_ref::<Result<String, Box<dyn std::error::Error>>>() {
            // Correct behavior: the fixed version should return an error.
            assert!(fixed_result.is_err(), "Expected an error for invalid input in the fixed version");
            let err_msg = fixed_result.as_ref().err().unwrap().to_string();
            assert!(err_msg.contains("Declared length exceeds input size"), "Unexpected error message: {}", err_msg);
        } else if let Some(vulnerable_output) = (&output as &dyn Any).downcast_ref::<String>() {
            // Vulnerable version erroneously returns a String.
            panic!("Function did not validate input properly, returned: {}", vulnerable_output);
        } else {
            panic!("Unexpected return type from process");
        }
    }
}