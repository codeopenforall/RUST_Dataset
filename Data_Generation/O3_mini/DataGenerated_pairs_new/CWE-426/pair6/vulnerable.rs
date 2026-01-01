//////////////////////////////////////////////////////////////////////////////////////////////////////
// Vulnerable Code - This code uses an unverified relative command name ("malicious_cmd"), which, when 
// executed using Command::new in an unsafe block, can be hijacked by an attacker controlling the 
// environment’s search path. The use of unsafe code combined with concurrent thread spawning amplifies 
// the risk of CWE-426 (Untrusted Search Path). Note that in a real-world scenario, an attacker might 
// plant a malicious binary with that name in a directory that appears earlier in PATH.
//////////////////////////////////////////////////////////////////////////////////////////////////////

use std::process::Command;
use std::sync::Arc;
use std::thread;

trait Runner {
    fn run(&self) -> bool;
}

struct Handler {
    exec_path: Arc<String>,
}

impl Runner for Handler {
    fn run(&self) -> bool {
        // Unsafe block that blindly executes a command from a relative path.
        unsafe {
            match Command::new(&*self.exec_path).output() {
                Ok(output) => output.status.success(),
                Err(_) => false,
            }
        }
    }
}

fn spawn_task() -> bool {
    // Vulnerable: using a relative command name without providing an absolute path.
    // This may cause the execution of a malicious binary if the search path is compromised.
    let cmd = Arc::new(String::from("malicious_cmd"));
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