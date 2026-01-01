//////////////////////////////////////////////////////////////////////////
// Vulnerability demonstration using unsafe command execution with shell
//////////////////////////////////////////////////////////////////////////
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

struct Executor {
    input: String,
    state: Arc<Mutex<i32>>,
}

impl Executor {
    fn new(input: String) -> Self {
        Executor { input, state: Arc::new(Mutex::new(0)) }
    }
    
    fn run(&self) -> String {
        // Vulnerability: Directly embedding untrusted input in a shell command.
        // This formulation permits an attacker to inject extra shell commands.
        let shell_command = format!("echo {}", self.input);  // <<< CWE-088 vulnerability point
        let output = unsafe {
            // Unsafe block used for command execution via the shell.
            Command::new("sh")
                .arg("-c")
                .arg(shell_command)
                .output()
                .expect("failed to execute process")
        };
        // Simulated shared state update with unsafe pointer dereference.
        let ptr = Arc::as_ptr(&self.state) as *mut Mutex<i32>;
        unsafe {
            // Unsynchronized access in an unsafe block – contributes to overall insecurity.
            let _guard = (*ptr).lock().unwrap();
        }
        String::from_utf8_lossy(&output.stdout).into_owned()
    }
}

fn simulate() -> String {
    // The malicious input contains a shell metacharacter (";") to trigger command injection.
    let executor = Executor::new(String::from("injected_text; echo injection"));
    let state_clone = executor.state.clone();
    let handle = thread::spawn(move || {
        // Another thread unsafely accesses shared state.
        let mut num = state_clone.lock().unwrap();
        *num += 1;
    });
    let result = executor.run();
    handle.join().unwrap();
    result
}

fn main() {
    let result = simulate();
    println!("Result: {}", result);
}