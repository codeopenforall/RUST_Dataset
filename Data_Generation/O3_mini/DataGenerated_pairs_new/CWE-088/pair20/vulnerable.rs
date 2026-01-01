////////////////////////////////////////////////////////////
// Vulnerable implementation for OS command injection CWE-088
////////////////////////////////////////////////////////////
use std::process::Command;
use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use std::thread;

struct Executor;

impl Executor {
    pub fn run(input: &str) -> Result<String, String> {
        // This function unsafely converts the input and constructs a shell
        // command without sanitization. An attacker can inject extra commands.
        unsafe {
            // Convert the input into a C-compatible string (unsafe if input contains NUL bytes)
            let raw = CString::new(input).map_err(|e| e.to_string())?;
            let ptr: *const c_char = raw.as_ptr();
            // Unsafely build a command string using the pointer;
            // the untrusted input is injected directly into the shell command.
            let cmd_string = format!("echo {}", CStr::from_ptr(ptr).to_string_lossy());
            let output = Command::new("sh")
                .arg("-c")
                .arg(cmd_string)
                .output()
                .map_err(|e| e.to_string())?;
            let result = String::from_utf8_lossy(&output.stdout).to_string();
            Ok(result.trim().to_string())
        }
    }
}

fn main() {
    // Read input from command line arguments.
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: program <input>");
        return;
    }
    // Spawn a thread to simulate concurrent execution.
    let user_input = args[1].clone();
    let handle = thread::spawn(move || {
        match Executor::run(&user_input) {
            Ok(res) => println!("{}", res),
            Err(e) => println!("Error: {}", e),
        }
    });
    handle.join().unwrap();
}