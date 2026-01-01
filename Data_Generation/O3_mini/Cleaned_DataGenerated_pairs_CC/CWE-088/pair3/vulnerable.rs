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
            unsafe {
                let s = data.lock().unwrap();
                let ptr = s.as_ptr();
                let raw_str = std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, s.len()));
                let command_str = format!("echo {}", raw_str);
                let output = Command::new("sh")
                    .arg("-c")
                    .arg(command_str)
                    .output();
                output
                    .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
                    .map_err(|e| e.to_string())
            }
        });
        handle.join().unwrap()
    }
}
fn main() {
    let input = "echo safe_input";
    let executor = Executor::new(input);
    match executor.run() {
        Ok(result) => println!("Result: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}
