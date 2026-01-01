///////////////////////////////////////////////////////
// Test Oracle Function using Rust's built-in test framework.
// This test is designed to catch sensitive information leaks.
// For the vulnerable version, the panic/error messages include "SuperSecretKey123"
// and the test will fail, whereas the fixed version uses generic messages and will pass.
///////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::panic;
    use std::sync::Arc;
    use std::error::Error;

    // A minimal Server and AppConfig interface expected by the test.
    // For testing purposes, ensure these symbols are accessible from the respective binaries.
    // The test will call process_request with an empty input to force an error.

    // The oracle test function.
    #[test]
    fn test_sensitive_data_exposure() {
        // Instantiate a Server object with known secret.
        let secret = "SuperSecretKey123";
        // Instead of calling main (which spawns threads and panics), we instantiate the object directly.
        let server = crate::Server::new(secret);

        // Call process_request to generate an error.
        let err_result = server.process_request("");
        match err_result {
            Ok(_) => panic!("Expected error was not returned."),
            Err(e) => {
                let err_str = e.to_string();
                // Test contract:
                // For the vulnerable code, the error message would include the secret.
                // For the fixed code, it will not.
                if err_str.contains(secret) {
                    panic!("Sensitive data exposed in error message.");
                }
                // Otherwise, the fixed code passes.
                assert!(true);
            }
        }
    }
}