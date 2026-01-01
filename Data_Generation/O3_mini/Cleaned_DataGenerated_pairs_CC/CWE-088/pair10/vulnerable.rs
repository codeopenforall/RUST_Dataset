use std::process::Command;
use std::thread;
use std::sync::{Arc, Mutex};
fn run(input: &str) -> Result<String, String> {
    unsafe {
        let ptr = input as *const str;
        let unsafe_input = &*ptr;
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!("echo {}", unsafe_input))
            .output()
            .map_err(|e| format!("Failed to execute: {}", e))?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err("Command executed with error".into())
        }
    }
}
fn main() {
    let shared_data = Arc::new(Mutex::new(String::from("Hello")));
    let data_clone = shared_data.clone();
    let handle = thread::spawn(move || {
        let result = run(&data_clone.lock().unwrap());
        match result {
            Ok(out) => println!("Output: {}", out),
            Err(e) => println!("Error: {}", e),
        }
    });
    handle.join().unwrap();
}
