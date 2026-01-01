/*
   Test oracle function for verifying sensitive data exposure.

   This unit test calls the "process" function with an invalid input.
   It then asserts that the error message does not contain the hardcoded secret string "S3cr3tP@ss".
   In the vulnerable version, the test will detect the secret and fail.
   In the corrected version, the test will pass as no sensitive data is disclosed.
*/
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn sensitive_data_not_exposed() {
        // Setup a data store with a sensitive secret.
        let store = Arc::new(Mutex::new(Data::new("S3cr3tP@ss")));
        // Use an input that triggers the error branch.
        let res = process("invalid_input", &store);
        // Expect an error result.
        match res {
            Ok(_) => panic!("Expected error, but got success."),
            Err(msg) => {
                // The error message should not include the sensitive secret.
                assert!(!msg.contains("S3cr3tP@ss"), "Error message leaks sensitive information!");
            }
        }
    }
}