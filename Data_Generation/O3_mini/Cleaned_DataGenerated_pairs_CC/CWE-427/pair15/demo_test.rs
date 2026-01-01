///////////////////////////////////////////////
// Test Oracle
///////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::env;

    // This test sets the attacker-controlled environment variable which causes the vulnerable version
    // to spawn an external command with a manipulated PATH. The vulnerable build will output "vulnerable"
    // and fail the assertion, while the fixed build will ignore this variable and output "secure".
    #[test]
    fn test_oracle() {
        // Force the MALICIOUS_PATH to a non-standard value.
        env::set_var("MALICIOUS_PATH", "/malicious/path");

        // Run the compiled binary (assumed to be the current executable).
        let output = Command::new(env::current_exe().unwrap())
            .output()
            .expect("Failed to execute binary");

        let output_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
        // The correct behavior is to always output "secure".
        assert_eq!(output_str, "secure", "Oracle test failed: got output '{}'", output_str);
    }
}