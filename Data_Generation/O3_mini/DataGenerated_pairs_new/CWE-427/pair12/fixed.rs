///////////////////////////////////////////////////////////////////////////////
// Secure version addressing CWE-427 by using a trusted, fixed search path.
// The code omits the unsafe conversion of user-controlled environment variables,
// and it replaces the PATH with a known safe value to ensure that the command is
// executed in a controlled environment.
///////////////////////////////////////////////////////////////////////////////

use std::env;
use std::process::Command;
use std::thread;
use std::time::Duration;

/// A helper structure for executing commands safely.
struct SafeExecutor;

impl SafeExecutor {
    // Instead of retrieving the PATH from the environment, we return a fixed, well-known PATH.
    fn trusted_path() -> String {
        "/usr/bin:/bin".to_string()
    }

    // Spawns a command while overriding the environment PATH with a trusted value.
    fn launch() -> Result<String, String> {
        let safe_path = Self::trusted_path();
        let output = Command::new("echo")
            .env("PATH", safe_path)
            .arg("hello")
            .output();
        match output {
            Ok(result) => {
                let out_str = String::from_utf8_lossy(&result.stdout).to_string();
                Ok(out_str)
            },
            Err(e) => Err(format!("Command execution failed: {:?}", e)),
        }
    }
}

fn main() {
    // Even though a concurrent thread is still running, its modifications will be ignored.
    let modifier = thread::spawn(|| {
        // An attacker may still attempt to change the PATH, but it will not affect our trusted setting.
        for i in 0..5 {
            env::set_var("PATH", format!("/malicious/path{}", i));
            thread::sleep(Duration::from_millis(40));
        }
    });

    // Wait for the modification thread to complete.
    modifier.join().unwrap();

    // Launch the command using the fixed, trusted search path.
    match SafeExecutor::launch() {
        Ok(result) => println!("Output: {}", result.trim()),
        Err(err) => println!("Error: {}", err),
    }
}