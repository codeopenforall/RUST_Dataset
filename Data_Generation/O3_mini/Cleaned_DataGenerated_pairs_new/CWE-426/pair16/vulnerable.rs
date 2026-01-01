use std::process::Command;
use std::sync::Arc;
use std::thread;
struct Loader {
    command: String,
}
impl Loader {
    fn new() -> Self {
        Loader {
            command: "example_app".to_string(),
        }
    }
    fn run(&self) -> Result<String, String> {
        unsafe {
            let output = Command::new(&self.command)
                .arg("--version")
                .output()
                .map_err(|e| format!("Failed execution: {}", e))?;
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        }
    }
    fn is_secure(&self) -> bool {
        self.command.starts_with("/")
    }
}
fn main() {
    let loader = Arc::new(Loader::new());
    let loader_clone = Arc::clone(&loader);
    let handle = thread::spawn(move || {
        match loader_clone.run() {
            Ok(output) => println!("Output: {}", output),
            Err(err) => eprintln!("Error: {}", err),
        }
    });
    handle.join().expect("Thread panicked");
}
