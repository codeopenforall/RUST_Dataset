#[cfg(test)]
mod tests {
    use std::env;
    use std::process::Command;
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    // The following test oracle is designed to validate the security fix.
    // When TEST_MODE is set in the environment, the vulnerable version returns an error,
    // whereas the secure version returns a successful output ("Secure execution").
    //
    // Note: This test expects that the Loader struct and its run() method are defined in the crate root.
    #[test]
    fn verify_loader_execution() {
        // Indicate test mode so that the command is simulated.
        env::set_var("TEST_MODE", "1");

        // Instantiate the Loader from the module under test.
        let secure_instance = crate::Loader::new();
        let secure_result = secure_instance.run();

        // In the fixed (secure) implementation, a success value is returned.
        assert_eq!(secure_result.unwrap(), "Secure execution");

        // To simulate the vulnerable behavior, we temporarily construct a Loader with a relative path.
        struct InsecureLoader {
            command: String,
        }
        impl InsecureLoader {
            fn new() -> Self {
                InsecureLoader { command: String::from("vuln_cmd_exe") }
            }
            fn run(&self) -> Result<String, String> {
                // In test mode, the insecure loader simulates failure.
                if std::env::var("TEST_MODE").is_ok() {
                    return Err(String::from("Insecure execution: relative path used"));
                }
                let cmd_clone = self.command.clone();
                let ptr = unsafe { cmd_clone.as_ptr().offset(0) };
                unsafe {
                    let bytes = std::slice::from_raw_parts(ptr, cmd_clone.len());
                    let loaded_cmd = String::from_utf8_lossy(bytes).into_owned();
                    let output = Command::new(loaded_cmd)
                        .arg("--version")
                        .output();
                    match output {
                        Ok(o) => Ok(String::from_utf8_lossy(&o.stdout).into_owned()),
                        Err(e) => Err(format!("Process launch failed: {}", e)),
                    }
                }
            }
        }
        let insecure_instance = InsecureLoader::new();
        let insecure_result = insecure_instance.run();

        // In the vulnerable (insecure) implementation, the result should be an error.
        assert!(insecure_result.is_err(), "Expected vulnerability simulation to fail");
    }
}