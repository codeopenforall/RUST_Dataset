//////////////////////////////
// Test Oracle Function
//////////////////////////////
#[cfg(test)]
mod tests {
    use std::env;
    use std::process::Command;
    use std::str;

    // This integration test spawns the compiled binary using the current executable.
    // It sets the environment variables so that:
    // - CMD_NAME remains "echo" (an allowlisted command).
    // - PATH is set to a known secure value for the fixed version.
    // The fixed version prints output containing "fixed" in its argument,
    // while the vulnerable version prints "vulnerable".
    // Thus, the test asserts that the output from the binary contains the substring "fixed".
    #[test]
    fn integration_oracle() {
        // Determine the binary to run. This assumes the test environment was built with Cargo.
        // The env var CARGO_BIN_EXE_<name> is automatically set by Cargo when running integration tests.
        // Replace "example" with the actual binary name if different.
        let exe = option_env!("CARGO_BIN_EXE_example").unwrap_or("target/debug/example");

        // Set a secure PATH so that if the fixed version is run, it uses the secure path.
        // For the vulnerable version, manipulating PATH may lead to unexpected behavior.
        let output = Command::new(exe)
            .env("CMD_NAME", "echo")
            .env("PATH", "/usr/bin:/bin")
            .output()
            .expect("Failed to execute binary");

        let stdout = str::from_utf8(&output.stdout).expect("Invalid UTF-8 in output");

        // The fixed (secure) binary prints "fixed" as an argument.
        // Therefore, assert that the output contains "fixed".
        // In the vulnerable code, the printed argument will be "vulnerable", causing this test to fail.
        assert!(
            stdout.contains("fixed"),
            "Test failed: The output does not contain the secure marker."
        );
    }
}