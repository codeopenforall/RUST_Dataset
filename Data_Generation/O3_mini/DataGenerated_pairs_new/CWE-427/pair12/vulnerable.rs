///////////////////////////////////////////////////////////////////////////////
// A complex example demonstrating CWE-427 (Uncontrolled Search Path Element)
// The code below spawns an OS command using an environment variable for the PATH,
// which can be modified concurrently by an external actor. The use of an unsafe
// block to get a C-string pointer from the PATH string, combined with concurrent
// modification, makes this code vulnerable.
///////////////////////////////////////////////////////////////////////////////

use std::env;
use std::ffi::CStr;
use std::process::Command;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

/// A helper structure for executing commands.
struct Executor;

impl Executor {
    // Returns the PATH environment variable by unsafely converting it to a C string.
    // This function is vulnerable because it does not validate the contents of PATH
    // and uses unsafe to reinterpret the string data.
    fn retrieve_path() -> Option<String> {
        if let Ok(val) = env::var("PATH") {
            // UNSAFE: Casting the Rust string's memory to a C string pointer without bounds checks.
            unsafe {
                let ptr = val.as_ptr() as *const i8;
                // The underlying memory may not be properly null-terminated if modified concurrently.
                let cstr = CStr::from_ptr(ptr);
                return cstr.to_str().ok().map(|s| s.to_string());
            }
        }
        None
    }

    // Spawns a command that uses the unsanitized PATH environment variable.
    fn launch() -> Result<String, String> {
        if let Some(search_path) = Self::retrieve_path() {
            // The unsanitized search_path is used to override the environment PATH.
            // An attacker controlling PATH may coerce the command to load an unintended binary.
            let output = Command::new("echo")
                .env("PATH", search_path)
                .arg("hello")
                .output();
            match output {
                Ok(result) => {
                    let out_str = String::from_utf8_lossy(&result.stdout).to_string();
                    Ok(out_str)
                },
                Err(e) => Err(format!("Command execution failed: {:?}", e)),
            }
        } else {
            Err("PATH environment variable not set.".to_string())
        }
    }
}

fn main() {
    // Simulate an attacker concurrently modifying the PATH environment variable.
    let shared_flag = Arc::new(Mutex::new(()));
    let flag_clone = Arc::clone(&shared_flag);
    let modifier = thread::spawn(move || {
        let _lock = flag_clone.lock().unwrap();
        // Rapidly change the PATH to dangerous values.
        for i in 0..5 {
            // The attacker-controlled input injected into PATH.
            env::set_var("PATH", format!("/malicious/path{}", i));
            thread::sleep(Duration::from_millis(40));
        }
    });

    // Wait for the thread to modify the environment.
    modifier.join().unwrap();

    // Launch the command using the vulnerable retrieval of PATH.
    match Executor::launch() {
        Ok(result) => println!("Output: {}", result.trim()),
        Err(err) => println!("Error: {}", err),
    }
}