use std::env;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
struct Executor;
impl Executor {
    fn new() -> Self {
        Executor
    }
    fn run_exec(&self) -> String {
        let secure_path = "/usr/bin".to_string();
        let output = Command::new("echo")
            .arg("secure")
            .env("PATH", secure_path)
            .stdout(Stdio::piped())
            .output()
            .expect("failed to execute process");
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    }
}
fn main() {
    let executor = Arc::new(Executor::new());
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];
    for _ in 0..4 {
        let executor_clone = Arc::clone(&executor);
        let results_clone = Arc::clone(&results);
        let handle = thread::spawn(move || {
            let result = executor_clone.run_exec();
            results_clone.lock().unwrap().push(result);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let final_result = results.lock().unwrap().get(0).cloned().unwrap_or_default();
    println!("{}", final_result);
}
