#[cfg(test)]
mod tests {
    use std::env;
    use std::fs;
    use std::process::Command;

    // This test assumes that the vulnerable and fixed versions are built as separate binaries
    // named "vulnerable_bin" and "fixed_bin" respectively.
    // The test sets the "SIM_FAILURE" environment variable to simulate a file write failure.
    // The secure (fixed) version should detect the error and exit with a non-zero status code.
    // The vulnerable version, which ignores the error, will incorrectly exit with success.

    #[test]
    fn test_unchecked_return_value() {
        // Test vulnerable binary
        let output_vuln = Command::new("vulnerable_bin")
            .env("SIM_FAILURE", "1")
            .output()
            .expect("Failed to execute vulnerable binary");
        // In the insecure variant, the error is ignored and the process exits with 0,
        // which is unsafe since the failure is not propagated.
        assert!(
            !output_vuln.status.success(),
            "Insecure implementation should not silently succeed when a critical operation fails"
        );

        // Test fixed binary
        let output_fixed = Command::new("fixed_bin")
            .env("SIM_FAILURE", "1")
            .output()
            .expect("Failed to execute fixed binary");
        // The secure implementation propagates the error and exits with a non-zero status,
        // correctly indicating the failure.
        assert!(
            !output_fixed.status.success(),
            "Secure implementation must detect failure and exit with an error"
        );
    }
}