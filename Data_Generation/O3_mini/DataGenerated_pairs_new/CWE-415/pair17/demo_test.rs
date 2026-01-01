#[cfg(test)]
mod tests {
    use std::process::Command;
    #[test]
    fn test_resource_management() {
        // Execute the compiled binary.
        // For the corrected binary, the output should reliably contain "Value: 500".
        // For the vulnerable version, the double free may cause a crash, abnormal termination, or invalid output.
        let output = Command::new("./target/debug/my_app")
            .output()
            .expect("Failed to execute process");
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Value: 500"),
                "Expected 'Value: 500' in output, got: {}", stdout);
    }
}