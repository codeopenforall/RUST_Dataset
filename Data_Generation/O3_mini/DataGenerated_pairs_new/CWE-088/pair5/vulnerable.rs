////////////////////////////////////////////////////////////////////////////////////////////////////
// This Rust code mimics a real-world OS command injection vulnerability (CWE-088).
// It uses unsafe string reconstruction and concurrency with Arc/Mutex and threads.
// The vulnerability occurs when an untrusted input string is injected directly into a shell command
// using format!(), allowing an attacker to inject arbitrary commands.
////////////////////////////////////////////////////////////////////////////////////////////////////
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::error::Error;

struct Executor {
    data: String,
}

impl Executor {
    fn new(data: String) -> Self {
        Executor { data }
    }

    // This function unsafely reconstructs the input and directly injects it into the shell command.
    // The unsanitized command string allows OS command injection if the input contains metacharacters.
    fn run(&self) -> Result<String, Box<dyn Error>> {
        // Unsafe block: reconstruct string from raw parts.
        let ptr = self.data.as_ptr();
        let len = self.data.len();
        let dangerous_input = unsafe {
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, len))
        };
        // Vulnerability: untrusted input is injected without validation into a shell command.
        // CWE-088: Injection via shell metacharacters (e.g. ";", "&", "|", etc.)
        let cmd_str = format!("echo safe && {}", dangerous_input);
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd_str)
            .output()?;
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(stdout)
    }
}

fn main() {
    // Accept input from the first command-line argument; if absent, use "default".
    let input = std::env::args().nth(1).unwrap_or_else(|| "default".to_string());
    let executor = Executor::new(input);
    let shared_exe = Arc::new(Mutex::new(executor));

    // Spawn a thread that executes the command.
    let shared_clone = Arc::clone(&shared_exe);
    let handle = thread::spawn(move || {
        let exe = shared_clone.lock().unwrap();
        exe.run().unwrap_or_else(|e| {
            eprintln!("execution error: {}", e);
            "error".to_string()
        })
    });

    let result = handle.join().unwrap();
    println!("{}", result);
}