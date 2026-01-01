use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
struct Executor;
impl Executor {
    fn run_command(&self, input: &str) -> Result<String, String> {
        if !input.chars().all(|c| c.is_ascii_alphanumeric() || c.is_whitespace()) {
            return Err("Invalid characters in input".to_string());
        }
        let output = Command::new("echo")
            .arg(input)
            .output()
            .map_err(|e| e.to_string())?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }
}
fn concurrent_exec(input: &str) -> Vec<Result<String, String>> {
    let executor = Arc::new(Mutex::new(Executor));
    let mut handles = vec![];
    for _ in 0..4 {
        let exec = Arc::clone(&executor);
        let input_data = input.to_string();
        let handle = thread::spawn(move || {
            let exec_locked = exec.lock().unwrap();
            exec_locked.run_command(&input_data)
        });
        handles.push(handle);
    }
    handles.into_iter().map(|h| h.join().unwrap()).collect()
}
fn main() {
    let safe_input = "Hello World";
    let results = concurrent_exec(safe_input);
    for res in results {
        match res {
            Ok(out) => println!("{}", out),
            Err(e) => eprintln!("{}", e),
        }
    }
}
