#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::str;

    // This test assumes that two binaries are produced:
    // one corresponding to the vulnerable sample ("vuln_app")
    // and one corresponding to the corrected sample ("safe_app").
    // The test spawns each binary with a malicious input to verify
    // that the vulnerable version exhibits abnormal behavior (by
    // either crashing or producing unexpected output), while the
    // corrected version produces the expected safe output.
    #[test]
    fn test_format_string_handling() {
        // Malicious input with a format specifier that the vulnerable code may misinterpret.
        let malicious = "test %s";

        // Execute the vulnerable binary.
        let vuln_output = Command::new("./vuln_app")
            .arg(malicious)
            .output()
            .expect("Failed to execute vulnerable binary");

        // Execute the safe binary.
        let safe_output = Command::new("./safe_app")
            .arg(malicious)
            .output()
            .expect("Failed to execute safe binary");

        // Convert outputs from bytes to strings.
        let vuln_stdout = str::from_utf8(&vuln_output.stdout).unwrap_or("");
        let safe_stdout = str::from_utf8(&safe_output.stdout).unwrap_or("");

        // In the corrected binary, the output will exactly match the user input.
        // In the vulnerable binary, using the user input as the format string can
        // result in undefined behavior. We assert that the vulnerable binary's output
        // does not simply equal the literal input (or that it fails).
        let expected = malicious.trim();

        // Check if the safe binary produced the expected output.
        assert_eq!(safe_stdout.trim(), expected, "Safe binary did not produce expected output");

        // For the vulnerable binary, either the process should fail (non-zero exit code)
        // or its output should not match the expected string.
        if vuln_output.status.success() {
            assert_ne!(vuln_stdout.trim(), expected,
                "Vulnerable binary unexpectedly produced the safe output");
        } else {
            // When the vulnerable binary crashes, it is acceptable as it shows the issue.
            assert!(!vuln_output.status.success(), "Vulnerable binary should not succeed with malicious input");
        }
    }
}