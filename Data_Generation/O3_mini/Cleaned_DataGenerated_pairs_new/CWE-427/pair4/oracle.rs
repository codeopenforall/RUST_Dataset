-------------------------------------------------
#[cfg(test)]
mod tests {
    use std::process::Command;
    #[test]
    fn check_search_path_sanitization() {
        let malicious_input = "/tmp/malicious";
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
