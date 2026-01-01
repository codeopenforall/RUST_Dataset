#[cfg(test)]
mod tests {
    use std::env;
    use std::process::Command;
    use std::str;
    #[test]
    fn integration_oracle() {
        let exe = option_env!("CARGO_BIN_EXE_example").unwrap_or("target/debug/example");
        let output = Command::new(exe)
            .env("CMD_NAME", "echo")
            .env("PATH", "/usr/bin:/bin")
            .output()
            .expect("Failed to execute binary");
        let stdout = str::from_utf8(&output.stdout).expect("Invalid UTF-8 in output");
        assert!(
            stdout.contains("fixed"),
            "Test failed: The output does not contain the secure marker."
        );
    }
}
