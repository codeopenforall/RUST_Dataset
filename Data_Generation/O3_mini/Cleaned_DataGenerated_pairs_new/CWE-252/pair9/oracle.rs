#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::str;
    #[test]
    fn oracle() {
        let output = Command::new("./target/debug/program")
            .arg("150")
            .output()
            .expect("Failed to execute process");
        let stdout = str::from_utf8(&output.stdout).expect("Output not UTF-8");
        assert!(
            stdout.contains("Failure"),
            "Test failed: expected a failure output, got: {}",
            stdout
        );
    }
}
