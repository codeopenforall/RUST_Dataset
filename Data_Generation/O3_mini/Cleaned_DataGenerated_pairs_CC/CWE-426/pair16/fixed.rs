use std::process::Command;
use std::sync::Arc;
use std::thread;
use std::path::Path;
struct Loader {
    command: String,
}
impl Loader {
    fn new() -> Self {
        Loader {
            command: "/usr/bin/example_app".to_string(),
        }
    }
    fn run(&self) -> Result<String, String> {
        if !self.is_secure() {
            return Err("Insecure command path detected".to_string());
        }
        unsafe {
            let output = Command::new(&self.command)
                .arg("--version")
                .output()
                .map_err(|e| format!("Failed execution: {}", e))?;
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        }
    }
    fn is_secure(&self) -> bool {
        Path::new(&self.command).is_absolute()
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
