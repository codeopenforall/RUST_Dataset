//////////////////////////////////////////////
// Test oracle function for both executables  //
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::str;

    // This helper function executes the compiled binary.
    // exe_path should be the path to the compiled binary (vulnerable or fixed).
    // The CUSTOM_PATH environment variable is set to simulate an attacker-controlled input.
    fn run_app(exe_path: &str, custom_path: &str) -> String {
        let output = Command::new(exe_path)
            .env("CUSTOM_PATH", custom_path)
            .output()
            .expect("failed to execute process");
        // Trim extra whitespace/newlines.
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    }

    #[test]
    fn test_app_output() {
        // For the vulnerable binary:
        // With CUSTOM_PATH set to "attacker", the unsanitized update leads to PATH being compromised,
        // so the simulated command returns "ATTACK".
        let vulnerable_result = run_app("./vulnerable_exe", "attacker");
        assert_eq!(vulnerable_result, "ATTACK");

        // For the fixed binary:
        // With CUSTOM_PATH set to "attacker", the update_env function does not override PATH,
        // so the Worker::run() function returns the safe output "safe_output".
        let fixed_result = run_app("./fixed_exe", "attacker");
        assert_eq!(fixed_result, "safe_output");
    }
}