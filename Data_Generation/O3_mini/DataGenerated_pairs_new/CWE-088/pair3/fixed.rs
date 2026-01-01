use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

struct Executor {
    shared: Arc<Mutex<String>>,
}

impl Executor {
    fn new(cmd: &str) -> Self {
        Executor {
            shared: Arc::new(Mutex::new(cmd.to_string())),
        }
    }

    fn run(&self) -> Result<String, String> {
        let data = self.shared.clone();
        let handle = thread::spawn(move || {
            // Obtain the input safely without using unsafe raw pointer manipulation.
            let s = data.lock().unwrap();
            // Check for dangerous shell metacharacters to mitigate injection.
            if s.contains(';') || s.contains('&') || s.contains('|') {
                return Err("Invalid characters in input".to_string());
            }
            // Safely build the command, knowing the input has been validated.
            let command_str = format!("echo {}", s);
            let output = Command::new("sh")
                .arg("-c")
                .arg(command_str)
                .output();
            output
                .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
                .map_err(|e| e.to_string())
        });
        handle.join().unwrap()
    }
}

fn main() {
    // Accept only trusted input; unsafe metacharacter sequences will be rejected.
    let input = "safe_input";
    let executor = Executor::new(input);
    match executor.run() {
        Ok(result) => println!("Result: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}