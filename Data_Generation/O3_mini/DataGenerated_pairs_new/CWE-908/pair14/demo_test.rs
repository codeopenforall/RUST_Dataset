////////////////////////////////////////////////////////////////////////////////
// This test oracle is designed to run against the compiled library version of the
// code. It asserts that the loaded resource's description field is properly initialized.
// The oracle will panic for the vulnerable version and succeed for the corrected version.
////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::panic;

    // Helper function that retrieves the length of the description field.
    fn retrieve() -> Result<usize, &'static str> {
        // Call the load_resource function from the enclosing module.
        let res = super::load_resource(42);
        let len = res.description.len();
        if len == 0 {
            Err("description was not properly initialized")
        } else {
            Ok(len)
        }
    }

    #[test]
    fn test_oracle() {
        // Catch any panics resulting from uninitialized access.
        let outcome = panic::catch_unwind(|| retrieve());
        match outcome {
            Ok(Ok(len)) => {
                // Check for the expected value in the corrected version.
                assert_eq!(len, "Properly initialized".len());
            },
            _ => panic!("Test oracle fails: resource initialization is improper"),
        }
    }
}