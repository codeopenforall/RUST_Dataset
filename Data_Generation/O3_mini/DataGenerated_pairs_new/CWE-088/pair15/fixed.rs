//////////////////////////////
// Corrected Code Snippet
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

    // In this version, the command is executed without invoking an intermediate shell.
    // The untrusted input is passed as an argument to the "echo" binary, avoiding any shell
    // interpretation of special characters.
    fn run(&self) -> Result<String, String> {
        let output = Command::new("echo")
            .arg(&self.input)
            .output()
            .map_err(|e| format!("Failed to execute: {}", e))?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
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
    // The same untrusted input is used here but the command is executed safely
    // because no intermediate shell performs unwanted interpretation.
    let input = "safe_input; echo injected";
    let app = Application::new(input);
    match app.start() {
        Ok(output) => println!("Command output: {}", output),
        Err(err) => eprintln!("Error: {}", err),
    }
}