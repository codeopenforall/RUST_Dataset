use std::process::Command;
use std::thread;
use std::sync::{Arc, Mutex};
struct Launcher {
    binary: String,
}
impl Launcher {
    fn new(input: &str) -> Self {
        Self { binary: input.to_string() }
    }
    fn run(&self) -> Result<String, String> {
        let mut output = String::new();
        let result = unsafe {
            Command::new(&self.binary)
                .arg("--version")
                .output()
        };
        match result {
            Ok(o) => {
                output = String::from_utf8_lossy(&o.stdout).to_string();
                Ok(output)
            }
            Err(e) => Err(format!("Error launching command: {}", e))
        }
    }
}
pub fn check_command(input: &str) -> Result<String, String> {
    let instance = Launcher::new(input);
    instance.run()
}
fn execute_threads() -> i32 {
    let shared = Arc::new(Mutex::new(Launcher::new("untrusted_binary")));
    let mut handles = vec![];
    for _ in 0..2 {
        let thread_launcher = shared.clone();
        let handle = thread::spawn(move || {
            let guard = thread_launcher.lock().unwrap();
            let _ = guard.run();
        });
        handles.push(handle);
    }
    for h in handles {
        h.join().unwrap();
    }
    0
}
fn main() {
    std::process::exit(execute_threads());
}
