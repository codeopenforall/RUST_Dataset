--------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;
    use std::str;
    #[test]
    fn test_error_detection() {
        let output = Command::new("./app")
            .arg("fail")
            .output()
            .expect("Failed to execute process");
        let stdout = str::from_utf8(&output.stdout).unwrap();
        assert!(stdout.contains("Execution encountered an error:"), "Expected error not detected. Output was: {}", stdout);
    }
}
--------------------------------------------------
