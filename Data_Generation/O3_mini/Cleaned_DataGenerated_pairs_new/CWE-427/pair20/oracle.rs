#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::str;
    fn run_app(exe_path: &str, custom_path: &str) -> String {
        let output = Command::new(exe_path)
            .env("CUSTOM_PATH", custom_path)
            .output()
            .expect("failed to execute process");
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    }
    #[test]
    fn test_app_output() {
        let vulnerable_result = run_app("./vulnerable_exe", "attacker");
        assert_eq!(vulnerable_result, "ATTACK");
        let fixed_result = run_app("./fixed_exe", "attacker");
        assert_eq!(fixed_result, "safe_output");
    }
}
