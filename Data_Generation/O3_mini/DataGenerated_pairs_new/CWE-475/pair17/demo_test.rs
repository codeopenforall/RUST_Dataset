#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn integration_oracle() {
        // This test assumes that the compiled binary is named "demo"
        // and located in the current directory.
        // For the vulnerable code, the printed total will be 52, while for the fixed version, it will be 10.
        let output = Command::new("./demo")
            .output()
            .expect("Failed to execute binary");
        let stdout = String::from_utf8_lossy(&output.stdout);
        // The oracle expects the fixed output ("Total: 10"). If not found, the test fails.
        assert!(
            stdout.contains("Total: 10"),
            "Oracle failure: expected 'Total: 10', got: {}",
            stdout
        );
    }
}