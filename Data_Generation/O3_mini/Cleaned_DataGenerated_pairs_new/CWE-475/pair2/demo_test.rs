//////////////////////////////
// Test Oracle Function
//////////////////////////////
#[cfg(test)]
mod tests {
    use std::process::{Command, Stdio};
    use std::io::Write;
    use std::str;

    // This oracle assumes that the compiled binary is named "app".
    // The test will run "app" and inspect its standard output.
    // In the corrected version, the computed total should be "Computed total: 6".
    // In the vulnerable version, due to undefined behavior, the output will likely differ or be incorrect.
    #[test]
    fn test_computation() {
        let output = Command::new("./app")
            .stdin(Stdio::null())
            .output()
            .expect("Failed to execute process");
        let stdout = str::from_utf8(&output.stdout).expect("Output not UTF-8");
        // Look for the expected computed total.
        // The expected value is "Computed total: 6"
        assert!(stdout.contains("Computed total: 6"), "Test failed: expected total 6, got:\n{}", stdout);
    }
}