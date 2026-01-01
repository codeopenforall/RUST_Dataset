#[cfg(test)]
mod tests {
    use std::env;
    use std::process::Command;

    // This test oracle spawns the current executable with a malicious input.
    // The expected safe output for the fixed version should print the input literally,
    // while the vulnerable version may interpret the format specifiers and produce unintended output.
    #[test]
    fn format_string_oracle() {
        // Malicious input containing format specifiers.
        let malicious_input = "Test %x %x";
        let current_exe = env::current_exe().expect("Failed to get current executable");

        let output = Command::new(current_exe)
            .arg(malicious_input)
            .output()
            .expect("Failed to execute process");
        let stdout = String::from_utf8_lossy(&output.stdout);

        // In the fixed version, the output should exactly equal the malicious input.
        // In the vulnerable version, printf may replace the %x specifiers with arbitrary data.
        assert_eq!(
            stdout.trim(),
            malicious_input,
            "The output does not match the expected safe output; potential format string vulnerability detected."
        );
    }
}