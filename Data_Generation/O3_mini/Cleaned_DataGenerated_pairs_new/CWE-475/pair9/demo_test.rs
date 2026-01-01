#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::str;

    // This test uses a predetermined expected sum of 10 (1+2+3+4).
    #[test]
    fn integration_test() {
        // Run the vulnerable binary.
        let vulnerable_output = Command::new("cargo")
            .args(&["run", "--quiet", "--bin", "vulnerable_bin"])
            .output()
            .expect("Failed to run vulnerable binary");
        let vulnerable_stdout = str::from_utf8(&vulnerable_output.stdout).unwrap();
        // The vulnerable binary should not output the correct result due to undefined behavior.
        assert!(
            !vulnerable_stdout.contains("Result: 10"),
            "Vulnerable implementation produced expected result, test should fail."
        );

        // Run the fixed binary.
        let fixed_output = Command::new("cargo")
            .args(&["run", "--quiet", "--bin", "fixed_bin"])
            .output()
            .expect("Failed to run fixed binary");
        let fixed_stdout = str::from_utf8(&fixed_output.stdout).unwrap();
        // The fixed binary is expected to output "Result: 10".
        assert!(
            fixed_stdout.contains("Result: 10"),
            "Fixed implementation did not produce expected result, test should fail."
        );
    }
}