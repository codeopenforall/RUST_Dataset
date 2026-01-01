//////////////////////////////////////////////////////////////////////////////////////////////////////
// Fixed Code - This implementation mitigates CWE-426 by using an absolute, verified path for the target 
// executable. By resolving the full path ("/bin/true" on Unix or a proper absolute path on Windows), it 
// prevents an attacker from injecting a malicious executable. The rest of the logic remains the same.
//////////////////////////////////////////////////////////////////////////////////////////////////////

use std::process::Command;
use std::sync::Arc;
use std::thread;

trait Runner {
    fn run(&self) -> bool;
}

#[cfg(unix)]
fn absolute_path() -> String {
    // On Unix systems, use an absolute path to a trusted binary.
    String::from("/bin/true")
}

#[cfg(windows)]
fn absolute_path() -> String {
    // On Windows systems, adjust to a fully qualified path to a trusted executable.
    String::from("C:\\Windows\\System32\\cmd.exe")
}

struct Handler {
    exec_path: Arc<String>,
}

impl Runner for Handler {
    fn run(&self) -> bool {
        // Unsafe block using a secured absolute path for command execution.
        unsafe {
            match Command::new(&*self.exec_path).output() {
                Ok(output) => output.status.success(),
                Err(_) => false,
            }
        }
    }
}

fn spawn_task() -> bool {
    // Use an absolute, trusted path to avoid untrusted search path vulnerabilities.
    let cmd = Arc::new(absolute_path());
    let handler = Handler { exec_path: cmd };
    let handle = thread::spawn(move || handler.run());
    handle.join().unwrap_or(false)
}

fn main() {
    if spawn_task() {
        println!("Operation succeeded");
    } else {
        println!("Operation failed");
    }
}