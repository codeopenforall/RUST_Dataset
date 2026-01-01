//////////////////////////////////////////////////////////////////////////
// Vulnerability: OS Command Injection due to unsanitized user input
//////////////////////////////////////////////////////////////////////////
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Executor {
    // A configuration prefix used in command construction.
    config: Arc<String>,
    // A shared counter to mimic thread coordination.
    counter: Arc<Mutex<u32>>,
}

impl Executor {
    pub fn new(cfg: String) -> Self {
        Executor {
            config: Arc::new(cfg),
            counter: Arc::new(Mutex::new(0)),
        }
    }

    // This method constructs a shell command by directly injecting the untrusted input
    // into a shell invocation. An unsafe block is used to mimic legacy pointer manipulation.
    pub fn execute(&self, user_input: &str) -> Result<String, String> {
        // UNSAFE: Using raw pointer operations to get the config string.
        let config_str = unsafe {
            let ptr = self.config.as_ptr();
            // Construct a byte slice from the pointer.
            let slice = std::slice::from_raw_parts(ptr, self.config.len());
            std::str::from_utf8(slice).unwrap_or("")
        };

        // VULNERABLE: The untrusted user input is directly concatenated into a command
        // string and passed to the shell with "-c". Metacharacters in user_input will
        // be interpreted by the shell.
        let command_line = format!("echo {} {}", config_str, user_input);

        // Spawn a thread to simulate concurrent processing.
        let counter_clone = self.counter.clone();
        let handle = thread::spawn(move || {
            Command::new("sh")
                .arg("-c")
                .arg(command_line)
                .output()
        });

        // Update a shared counter (unrelated to the vulnerability).
        if let Ok(mut cnt) = self.counter.lock() {
            *cnt += 1;
        }

        // Collect the command output.
        match handle.join() {
            Ok(Ok(output)) => Ok(String::from_utf8_lossy(&output.stdout).to_string()),
            _ => Err("Execution failure".to_string()),
        }
    }
}

fn main() {
    let executor = Executor::new("PREFIX".to_string());
    // Trigger the injection vulnerability by including shell metacharacters.
    let res = executor.execute("hello; echo INJECTED");
    println!("Result: {:?}", res);
}