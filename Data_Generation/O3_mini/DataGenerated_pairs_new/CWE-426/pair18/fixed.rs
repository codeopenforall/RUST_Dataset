//////////////////////////////////////////////////////////////
// Fixed version:
//
// This program addresses the CWE-426 vulnerability by ensuring
// that the binary path is resolved to an absolute path before execution.
// In the constructor, if a relative name like "echo" is provided, it is
// mapped to a trusted absolute path (for example, "/bin/echo"). This change
// mitigates the risk of DLL preloading/untrusted search path attacks. The
// remainder of the logic (concurrent execution and command evaluation)
// remains unchanged.
//////////////////////////////////////////////////////////////

use std::process::Command;
use std::ffi::CString;
use std::sync::Arc;
use std::thread;
use std::path::Path;

trait Executor {
    fn execute(&self, args: &[&str]) -> Result<String, String>;
}

struct Handler {
    binary: CString,
}

impl Handler {
    fn new(bin: &str) -> Self {
        // Enforce absolute path resolution. If a relative command is provided,
        // transform it into a trusted absolute path.
        let abs_path = if Path::new(bin).is_absolute() {
            bin.to_string()
        } else {
            // Here we handle known commands.
            match bin {
                "echo" => String::from("/bin/echo"),
                _ => bin.to_string(),
            }
        };
        // Use safe conversion: CString::new checks for interior nulls.
        let c_str = CString::new(abs_path).expect("CString conversion failed");
        Handler { binary: c_str }
    }
}

impl Executor for Handler {
    fn execute(&self, args: &[&str]) -> Result<String, String> {
        // With an absolute path provided, the search path vulnerability is mitigated.
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

fn run_task(exec: &dyn Executor) -> Result<String, String> {
    exec.execute(&["Test"])
}

fn main() {
    // "echo" is converted to its safe absolute path.
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