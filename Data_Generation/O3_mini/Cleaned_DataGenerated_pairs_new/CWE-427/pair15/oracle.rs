#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::env;
    #[test]
    fn test_oracle() {
        env::set_var("MALICIOUS_PATH", "/malicious/path");
        let output = Command::new(env::current_exe().unwrap())
            .output()
            .expect("Failed to execute binary");
        let output_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
        assert_eq!(output_str, "secure", "Oracle test failed: got output '{}'", output_str);
    }
}
