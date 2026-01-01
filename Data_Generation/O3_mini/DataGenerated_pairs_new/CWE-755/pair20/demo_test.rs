//////////////////////////////////////////////////////////////////////////////////////////////////////
// This test oracle is common to both builds. It writes a malformed configuration file and calls the 
// adapter function 'execute'. In the vulnerable version, due to error swallowing, a default value (0) is returned,
// while in the fixed version an error is propagated. The test asserts that a non-valid computation is not accepted,
// thereby failing in the vulnerable build and passing in the fixed one.
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_oracle() {
        // Write a deliberately malformed configuration file.
        fs::write("config.txt", "invalid_number").expect("Failed to write config file");
        // Call the adapter function.
        let res = execute(5);
        match res {
            // In the vulnerable version, execute returns Ok(0) (default), which is incorrect.
            Ok(val) => {
                // Expect a non-zero valid result; if 0 is returned, then the error was swallowed.
                assert!(val != 0, "Test failed: received 0 indicating that errors were swallowed");
            },
            // In the fixed version, an error is correctly propagated.
            Err(_) => {
                assert!(true, "Properly detected configuration error");
            }
        }
        let _ = fs::remove_file("config.txt");
    }
}