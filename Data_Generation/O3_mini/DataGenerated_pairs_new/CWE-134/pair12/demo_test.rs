#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn test_behavior() {
        // This test compares the behavior of two binaries:
        // "./app_bad" is compiled from the dangerous version,
        // whereas "./app_good" is compiled from the corrected version.
        // The dangerous version is expected to crash or return a non-zero exit code
        // when fed a specially crafted input containing extra format specifiers.

        // Dangerous binary execution with a malicious format string.
        let bad_output = Command::new("./app_bad")
            .arg("Exploit %s %p")
            .output()
            .expect("Failed to execute app_bad");
        // Correct binary execution with the same input should succeed.
        let good_output = Command::new("./app_good")
            .arg("Exploit %s %p")
            .output()
            .expect("Failed to execute app_good");

        assert!(
            !bad_output.status.success(),
            "Bad version should fail with dangerous format string input"
        );
        assert!(
            good_output.status.success(),
            "Good version should succeed with dangerous format string input"
        );
    }
}