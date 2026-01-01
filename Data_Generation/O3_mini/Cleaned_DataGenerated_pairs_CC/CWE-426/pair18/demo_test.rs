//////////////////////////////////////////////////////////////
// Test Oracle Function:
//
// This test simulates a malicious environment by creating a temporary 
// directory with a fake "echo" binary that outputs "VULNERABLE" instead 
// of the expected "Test" response. It then prepends this directory to the 
// PATH environment variable. When the vulnerable version is run, it picks 
// up the malicious binary and returns "VULNERABLE\n", failing the test. 
// In contrast, the fixed version uses an absolute path (e.g. "/bin/echo"), 
// bypassing the malicious directory, and returns "Test\n", thus passing the test.
//////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::env;
    use std::path::PathBuf;
    #[cfg(unix)]
    use std::os::unix::fs::PermissionsExt;

    #[test]
    fn oracle_test() {
        // Create a temporary directory for a malicious "echo" binary.
        let mut temp_dir = env::temp_dir();
        temp_dir.push("malicious_echo");
        fs::create_dir_all(&temp_dir).expect("Failed to create temporary directory");
        let fake_echo = temp_dir.join("echo");
        
        // Write a simple shell script that outputs "VULNERABLE".
        let script = "#!/bin/sh\necho VULNERABLE\n";
        let mut file = File::create(&fake_echo).expect("Failed to create fake echo");
        file.write_all(script.as_bytes()).expect("Failed to write to fake echo");
        #[cfg(unix)]
        {
            let mut perms = fs::metadata(&fake_echo).unwrap().permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&fake_echo, perms).unwrap();
        }
        
        // Prepend the temporary directory to the current PATH.
        let original_path = env::var("PATH").unwrap_or_default();
        let new_path = format!("{}:{}", temp_dir.to_str().unwrap(), original_path);
        env::set_var("PATH", new_path);

        // Instantiate the handler using the vulnerable/fixed constructor.
        let instance = Handler::new("echo");
        let output = run_task(&instance).expect("Command execution failed");

        // The secure/fixed version will ignore the malicious PATH and produce "Test\n".
        // The vulnerable version will execute the malicious binary and produce "VULNERABLE\n".
        assert_eq!(output, "Test\n", "Test failed due to untrusted search path vulnerability.");
    }
}