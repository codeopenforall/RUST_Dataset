////////////////////////////////////////////////////////////
// Test Oracle for CWE-427: Uncontrolled Search Path Element
// This unit test sets the environment variable "MALICIOUS_PATH" to an unsafe value and
// then calls the resolve function. It asserts that for the secure version, the resolved path
// equals the safe default, while for the vulnerable version the unsafe value would be used.
////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use std::env;
    // To test, import the resolve_path function from the module.
    // Note: The function names and structure must be identical between the two compilable versions.
    use super::Application;

    #[test]
    fn test_path_resolution() {
        // Set the malicious environment variable to a dangerous value.
        env::set_var("MALICIOUS_PATH", "malicious/path");
        let app = Application;
        let resolved = app.resolve_path();

        // The secure version should ignore the malicious input and use a safe default.
        // For the vulnerable version, the resolved value would be "malicious/path".
        //
        // Adjust the expected value accordingly: The fixed version should yield "/usr/bin:/bin".
        assert_eq!(resolved, "/usr/bin:/bin", "Resolved PATH should be safe even if MALICIOUS_PATH is set maliciously");
    }
}