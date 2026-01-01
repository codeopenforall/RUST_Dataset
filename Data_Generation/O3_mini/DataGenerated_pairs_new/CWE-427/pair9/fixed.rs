//////////////////////////////////////////////////////////////////
// This Rust code has been revised to mitigate CWE-427 by validating
// any library path provided from external sources. The code only
// accepts library paths from a fixed whitelist instead of trusting
// the environment's CUSTOM_PATH variable. This prevents an attacker
// from injecting malicious directories into the search path.
//////////////////////////////////////////////////////////////////

use std::env;
use std::process::Command;
use std::sync::Arc;
use std::thread;

struct Executor<'a> {
    info: &'a str,
}

impl<'a> Executor<'a> {
    fn new(text: &'a str) -> Executor<'a> {
        Executor { info: text }
    }
    
    // In this secure version, we validate the incoming library path.
    // Only allowed paths from a whitelist are accepted.
    unsafe fn run_command(&self, lib: &str) -> Result<String, String> {
        // Whitelist for allowed library directories.
        let allowed = ["/usr/lib", "/usr/local/lib"];
        if !allowed.contains(&lib) {
            return Err("Library path not allowed".to_string());
        }
        
        let mut current = env::var("PATH").unwrap_or_default();
        // Although an unsafe block is maintained, the input has been validated.
        current.push_str(":");
        current.push_str(lib);
        env::set_var("PATH", &current);
        
        let output = Command::new("echo").arg(self.info).output();
        match output {
            Ok(out) => Ok(String::from_utf8_lossy(&out.stdout).to_string()),
            Err(e) => Err(format!("Command execution error: {}", e)),
        }
    }
}

fn main() {
    let executor = Executor::new("safe run");
    // Even though the CUSTOM_PATH environment variable is read,
    // the code only proceeds with allowed values.
    let lib_input = env::var("CUSTOM_PATH").unwrap_or_else(|_| "/usr/lib".to_string());
    
    let shared = Arc::new(executor);
    let cloned = Arc::clone(&shared);
    
    let handler = thread::spawn(move || {
        unsafe { cloned.run_command(&lib_input) }
    });
    
    match handler.join() {
        Ok(Ok(result)) => println!("Output: {}", result),
        _ => println!("Execution error occurred"),
    }
}