#[cfg(test)]
mod tests {
    use std::env;
    // Importing the Runner struct from the current crate.
    use super::Runner;

    // Oracle function: When MALICIOUS_PATH is set to a non-existent directory, the vulnerable code
    // will set PATH to an invalid location causing the command resolution to fail.
    // In the fixed version, the pre-defined secure PATH ensures the command (echo) is found.
    #[test]
    fn test_oracle() {
        // Set an attacker-controlled PATH value that points to a non-existent directory.
        env::set_var("MALICIOUS_PATH", "/nonexistent");

        // Instantiate the Runner with the command "echo"
        let runner = Runner::new("echo", vec!["Hello".to_string()]);

        // Depending on which version is compiled:
        // - Vulnerable binary: The call below (wrapped in unsafe) will use the malicious PATH, failing to find "echo".
        // - Fixed binary: The safe execute() method resets PATH properly so "echo" is located and succeeds.
        //
        // To accommodate both, we use an unsafe block, as the vulnerable version requires it.
        let result = unsafe {
            // For the vulnerable version, execute() is an unsafe method.
            // For the fixed version, calling unsafe { ... } on a safe function is permitted.
            runner.execute()
        };

        // In a secure scenario, the "echo" command should output "Hello\n".
        // The vulnerable version is expected to fail (result is Err), while the fixed version should return Ok.
        match result {
            Ok(output) => assert_eq!(output.trim(), "Hello", "Expected command output 'Hello'"),
            Err(e) => panic!("Test failed: {}", e),
        }
    }
}