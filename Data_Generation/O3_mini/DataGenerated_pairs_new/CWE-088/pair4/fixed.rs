/* 
   This program corrects the command injection vulnerability by ensuring that untrusted input is safely handled.
   Instead of constructing and passing an unsanitized command line to the shell, it sanitizes the input and 
   directly invokes the intended executable (echo) without a shell.
*/
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

// Function that simulates handling an untrusted string and safely executes a command.
fn safe_run() -> String {
    // In a real scenario, the untrusted input would be provided by the user.
    let untrusted = "echo test; echo injected";
    // Sanitize the input by allowing only alphanumeric characters and whitespace.
    // This filtering prevents shell metacharacters from being interpreted.
    let clean: String = untrusted
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect();
    
    // Instead of invoking the shell, directly execute the "echo" command with the sanitized argument.
    let output = Command::new("echo")
        .arg(clean)
        .output()
        .expect("failed to execute process");
    
    String::from_utf8_lossy(&output.stdout).to_string()
}

// Concurrency is maintained by using the same Executor structure.
struct Executor {
    data: Arc<Mutex<String>>,
}

impl Executor {
    fn new() -> Self {
        Executor {
            data: Arc::new(Mutex::new(String::new())),
        }
    }

    fn run(&self) {
        let data_clone = Arc::clone(&self.data);
        thread::spawn(move || {
            // This call uses safe_run which sanitizes the input and avoids shell invocation.
            let res = safe_run();
            let mut guard = data_clone.lock().unwrap();
            *guard = res;
        })
        .join()
        .unwrap();
    }

    fn get_result(&self) -> String {
        let guard = self.data.lock().unwrap();
        guard.clone()
    }
}

// Expose a function for the test oracle to call.
pub fn run_app() -> String {
    let exec = Executor::new();
    exec.run();
    exec.get_result()
}

fn main() {
    // Execute the secure application flow.
    let out = run_app();
    println!("{}", out);
}