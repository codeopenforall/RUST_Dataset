use std::process::Command;
use std::sync::Arc;
use std::thread;
struct Executor {
    input: Arc<String>,
}
impl Executor {
    fn new(input: Arc<String>) -> Self {
        Executor { input }
    }
    fn run(&self) -> Result<String, String> {
        let untrusted = self.input.as_str();
        let output = Command::new("echo")
            .arg(untrusted)
            .output();
        match output {
            Ok(o) => Ok(String::from_utf8_lossy(&o.stdout).to_string()),
            Err(e) => Err(format!("Execution error: {}", e)),
        }
    }
}
fn main() {
    let raw_input = std::env::args().nth(1).unwrap_or_else(|| "default_input".to_string());
    let shared_input = Arc::new(raw_input);
    let runner = Executor::new(shared_input.clone());
    let handle = thread::spawn(move || {
        runner.run().unwrap_or_else(|_| "error".to_string())
    });
    let result = handle.join().expect("Thread panicked");
    println!("{}", result);
}
