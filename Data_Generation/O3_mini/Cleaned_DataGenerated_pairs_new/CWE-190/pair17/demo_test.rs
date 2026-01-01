#[cfg(test)]
mod tests {
    use std::process::Command;

    // This test function builds and runs both binaries.
    // It verifies that the vulnerable version produces an incorrect (wrapped) result,
    // while the corrected version produces the expected 5,000,000,000.
    #[test]
    fn test_multiplication() {
        let expected: u64 = 5_000_000_000;

        // Execute the vulnerable binary.
        let output_vulnerable = Command::new("cargo")
            .args(&["run", "--bin", "vulnerable"])
            .output()
            .expect("Failed to execute vulnerable binary");
        let stdout_vulnerable = String::from_utf8_lossy(&output_vulnerable.stdout);
        // Extract the last token on the printed line assuming the format "Result: <value>"
        let parsed_vulnerable = stdout_vulnerable
            .trim()
            .split_whitespace()
            .last()
            .expect("Failed to parse vulnerable output")
            .parse::<u64>()
            .expect("Failed to convert vulnerable result to u64");
        // The vulnerable version overflows and thus must NOT equal the expected correct value.
        assert_ne!(parsed_vulnerable, expected, "Vulnerable implementation produced the correct result unexpectedly");

        // Execute the fixed binary.
        let output_fixed = Command::new("cargo")
            .args(&["run", "--bin", "fixed"])
            .output()
            .expect("Failed to execute fixed binary");
        let stdout_fixed = String::from_utf8_lossy(&output_fixed.stdout);
        let parsed_fixed = stdout_fixed
            .trim()
            .split_whitespace()
            .last()
            .expect("Failed to parse fixed output")
            .parse::<u64>()
            .expect("Failed to convert fixed result to u64");
        // The fixed version must exactly match the expected correct value.
        assert_eq!(parsed_fixed, expected, "Fixed implementation did not produce the correct result");
    }
}