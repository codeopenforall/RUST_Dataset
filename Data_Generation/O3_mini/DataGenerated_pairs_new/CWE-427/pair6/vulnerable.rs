//////////////////////////////////////////////
// Vulnerable Sample Code
//////////////////////////////////////////////
use std::env;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;

struct Executor;

impl Executor {
    // This function constructs a command execution using an unsanitized PATH.
    // It simulates unsafe conversion of an attacker-controlled environment variable.
    fn run(&self, program: &str, args: &[&str]) -> std::io::Result<()> {
        // Unsafe block: reading and converting the PATH environment variable without proper validation.
        let unsafe_path = unsafe {
            let raw = env::var("PATH").unwrap_or_else(|_| String::new());
            // Directly converting to CString; if the PATH contains null bytes, this may lead to issues.
            CString::new(raw).unwrap()
        };

        // Passing the unsanitized PATH directly to the command environment.
        let mut cmd = Command::new(program);
        cmd.env("PATH", unsafe_path.to_str().unwrap())
           .args(args);

        // Launch the command within a separate thread to simulate concurrent execution.
        let shared_cmd = Arc::new(Mutex::new(cmd));
        let thread_cmd = Arc::clone(&shared_cmd);
        let handle = thread::spawn(move || {
            let mut guard = thread_cmd.lock().unwrap();
            // An attacker-controlled PATH could result in executing a rogue binary.
            guard.spawn().and_then(|mut child| child.wait())
        });
        // Propagate the result of the spawned command.
        handle.join().unwrap()
    }
}

fn main() {
    let executor = Executor;
    // The execution call is vulnerable because it relies on the external "PATH".
    executor.run("echo", &["Hello from unsafe execution!"]).unwrap();
}