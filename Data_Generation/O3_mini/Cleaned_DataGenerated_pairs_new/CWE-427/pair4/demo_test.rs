-------------------------------------------------
#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn check_search_path_sanitization() {
        // The test uses an attacker-controlled search path.
        // For the unsafe binary, this will lead to execution of the unsafe command, printing "unsafe_exec",
        // causing the test to fail. For the safe binary, the input is sanitized so it prints "safe_exec".
        let malicious_input = "/tmp/malicious";

        // Execute the compiled binary (assumed as "./app") with the malicious search path.
        let output = Command::new("./app")
            .arg(malicious_input)
            .output()
            .expect("Failed to execute the binary");

        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.contains("unsafe_exec") {
            panic!("Test failed: Uncontrolled search path element vulnerability exploited.");
        } else if stdout.contains("safe_exec") {
            assert!(true);
        } else {
            panic!("Test inconclusive: Unexpected output.");
        }
    }
}
-------------------------------------------------