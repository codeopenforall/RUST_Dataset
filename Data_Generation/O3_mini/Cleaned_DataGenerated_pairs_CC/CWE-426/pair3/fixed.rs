use std::process::Command;
use std::thread;
use std::sync::{Arc, Mutex};
use std::path::Path;
struct Runner {
    binary: String,
}
impl Runner {
    fn new(input: &str) -> Self {
        Self { binary: input.to_string() }
    }
    fn run(&self) -> Result<String, String> {
        let mut output = String::new();
        let bin_path = Path::new(&self.binary);
        if !bin_path.is_absolute() {
            return Err("Command must be absolute path".into());
        }
        let result = unsafe {
            Command::new(&self.binary)
                .arg("--version")
                .output()
        };
        match result {
            Ok(o) => {
                output = String::from_utf8_lossy(&o.stdout).to_string();
                Ok(output)
            },
            Err(e) => Err(format!("Error launching command: {}", e))
        }
    }
}
pub fn check_command(input: &str) -> Result<String, String> {
    let instance = Runner::new(input);
    instance.run()
}
fn execute_parallel() -> i32 {
    let shared = Arc::new(Mutex::new(Runner::new("/usr/bin/untrusted_binary")));
    let mut handles = vec![];
    for _ in 0..2 {
        let thread_runner = shared.clone();
        let handle = thread::spawn(move || {
            let guard = thread_runner.lock().unwrap();
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
    std::process::exit(execute_parallel());
}
