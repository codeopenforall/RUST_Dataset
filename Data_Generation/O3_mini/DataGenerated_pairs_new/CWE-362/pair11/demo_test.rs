#[cfg(test)]
mod tests {
    use std::process::Command;

    // This test oracle is designed to run two separate compiled binaries:
    // one built from the vulnerable implementation ("vuln_bin")
    // and one built from the corrected implementation ("fix_bin").
    // The vulnerable binary is expected to yield a final output differing from "Final value: 1",
    // while the fixed binary should output "Final value: 1".
    //
    // NOTE: Ensure that your build system names the output binaries appropriately.
    #[test]
    fn test_race_condition() {
        // Execute the vulnerable binary.
        let output_vuln = Command::new("./vuln_bin")
            .output()
            .expect("Failed to execute vulnerable binary");
        let stdout_vuln = String::from_utf8_lossy(&output_vuln.stdout);
        // Expect the vulnerable binary's output to NOT contain the expected final value.
        assert!(
            !stdout_vuln.contains("Final value: 1"),
            "Vulnerable code unexpectedly produced the expected result."
        );

        // Execute the fixed binary.
        let output_fixed = Command::new("./fix_bin")
            .output()
            .expect("Failed to execute fixed binary");
        let stdout_fixed = String::from_utf8_lossy(&output_fixed.stdout);
        // The fixed binary should produce the expected final value.
        assert!(
            stdout_fixed.contains("Final value: 1"),
            "Fixed code did not produce the expected final result."
        );
    }
}