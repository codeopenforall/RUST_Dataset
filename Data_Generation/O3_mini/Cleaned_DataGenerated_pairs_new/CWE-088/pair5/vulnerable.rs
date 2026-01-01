use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::error::Error;
struct Executor {
    data: String,
}
impl Executor {
    fn new(data: String) -> Self {
        Executor { data }
    }
    fn run(&self) -> Result<String, Box<dyn Error>> {
        let ptr = self.data.as_ptr();
        let len = self.data.len();
        let dangerous_input = unsafe {
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, len))
        };
        let cmd_str = format!("echo safe && {}", dangerous_input);
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd_str)
            .output()?;
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(stdout)
    }
}
fn main() {
    let input = std::env::args().nth(1).unwrap_or_else(|| "default".to_string());
    let executor = Executor::new(input);
    let shared_exe = Arc::new(Mutex::new(executor));
    let shared_clone = Arc::clone(&shared_exe);
    let handle = thread::spawn(move || {
        let exe = shared_clone.lock().unwrap();
        exe.run().unwrap_or_else(|e| {
            eprintln!("execution error: {}", e);
            "error".to_string()
        })
    });
    let result = handle.join().unwrap();
    println!("{}", result);
}
