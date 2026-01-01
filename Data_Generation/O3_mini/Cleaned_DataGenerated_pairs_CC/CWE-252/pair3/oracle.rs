#[cfg(test)]
mod tests {
    use std::env;
    use std::fs;
    use std::process::Command;
    #[test]
    fn test_unchecked_return_value() {
        let output_vuln = Command::new("vulnerable_bin")
            .env("SIM_FAILURE", "1")
            .output()
            .expect("Failed to execute vulnerable binary");
        assert!(
            !output_vuln.status.success(),
            "Insecure implementation should not silently succeed when a critical operation fails"
        );
        let output_fixed = Command::new("fixed_bin")
            .env("SIM_FAILURE", "1")
            .output()
            .expect("Failed to execute fixed binary");
        assert!(
            !output_fixed.status.success(),
            "Secure implementation must detect failure and exit with an error"
        );
    }
}
