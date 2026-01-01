//////////////////////////////
// Vulnerable Code Snippet
//////////////////////////////
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

struct Executor {
    input: String,
}

impl Executor {
    fn new(input: &str) -> Self {
        Executor {
            input: input.to_string(),
        }
    }

    // This function constructs a shell command by directly injecting untrusted input
    // into the command string. The use of an unsafe block further hides the direct call,
    // mimicking real-world misuse of unsafe code and concurrent shared usage.
    fn run(&self) -> Result<String, String> {
        let cmd = format!("echo {}", self.input);
        unsafe {
            let output = Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .output()
                .map_err(|e| format!("Failed to execute: {}", e))?;
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        }
    }
}

struct Application {
    executor: Arc<Mutex<Executor>>,
}

impl Application {
    fn new(input: &str) -> Self {
        Application {
            executor: Arc::new(Mutex::new(Executor::new(input))),
        }
    }

    fn start(&self) -> Result<String, String> {
        let executor_arc = Arc::clone(&self.executor);
        let handle = thread::spawn(move || {
            let executor = executor_arc.lock().unwrap();
            executor.run()
        });
        handle.join().unwrap()
    }
}

fn main() {
    // Simulated untrusted input that contains shell metacharacters.
    // An attacker-provided payload might inject extra commands through the shell.
    let input = "safe_input; echo injected";
    let app = Application::new(input);
    match app.start() {
        Ok(output) => println!("Command output: {}", output),
        Err(err) => eprintln!("Error: {}", err),
    }
}