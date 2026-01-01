//////////////////////////////////////////////////////////////
// Vulnerable version:
//
// This program defines a handler that runs an external command 
// using a relative command name ("echo") obtained via an unsafe 
// conversion to a C-style string. The command is executed inside 
// an unsafe block in a concurrent context. Using a relative 
// command name without an absolute path creates an Untrusted Search 
// Path vulnerability (CWE-426), since an attacker might inject a 
// malicious executable with the same name via a manipulated PATH.
//////////////////////////////////////////////////////////////

use std::process::Command;
use std::ffi::CString;
use std::sync::Arc;
use std::thread;

trait Executor {
    fn execute(&self, args: &[&str]) -> Result<String, String>;
}

struct Handler {
    binary: CString,
}

impl Handler {
    fn new(bin: &str) -> Self {
        // Unsafe conversion from a regular string to a CString.
        // Note: This does not verify that the command string is safe.
        let bytes = bin.as_bytes();
        let c_str = unsafe {
            // Using from_vec_unchecked bypasses any checking.
            CString::from_vec_unchecked(bytes.to_vec())
        };
        Handler { binary: c_str }
    }
}

impl Executor for Handler {
    fn execute(&self, args: &[&str]) -> Result<String, String> {
        unsafe {
            // POTENTIAL VULNERABILITY:
            // Using a relative command name opens the possibility for an attacker to
            // override the system command by placing a malicious executable earlier in
            // the search path.
            let mut cmd = Command::new(self.binary.to_str().unwrap());
            for arg in args {
                cmd.arg(arg);
            }
            match cmd.output() {
                Ok(output) => {
                    let result = String::from_utf8_lossy(&output.stdout).to_string();
                    Ok(result)
                }
                Err(e) => Err(format!("Execution error: {}", e)),
            }
        }
    }
}

fn run_task(exec: &dyn Executor) -> Result<String, String> {
    exec.execute(&["Test"])
}

fn main() {
    // The relative binary "echo" is used.
    let handler = Handler::new("echo");
    let shared = Arc::new(handler);
    let workers: Vec<_> = (0..2)
        .map(|_| {
            let exec = Arc::clone(&shared);
            thread::spawn(move || run_task(&*exec))
        })
        .collect();

    for worker in workers {
        match worker.join().unwrap() {
            Ok(output) => println!("Output: {}", output),
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}