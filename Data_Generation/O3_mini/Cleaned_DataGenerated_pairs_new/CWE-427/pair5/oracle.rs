#[cfg(test)]
mod tests {
    use std::process::Command;
    #[test]
    fn oracle() {
        let dangerous_env = "/tmp/malicious";
        let output_vul = Command::new("./vulnerable_bin")
            .env("INJECTED_PATH", dangerous_env)
            .output()
            .expect("Failed to run the vulnerable binary");
        assert_ne!(output_vul.status.code(), Some(0), "Vulnerable binary succeeded unexpectedly");
        let output_fix = Command::new("./fixed_bin")
            .env("INJECTED_PATH", dangerous_env)
            .output()
            .expect("Failed to run the fixed binary");
        assert_eq!(output_fix.status.code(), Some(0), "Fixed binary did not succeed as expected");
    }
}
