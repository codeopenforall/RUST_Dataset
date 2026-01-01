///////////////////////////////
// Vulnerable Version Example
///////////////////////////////
use std::process::Command;
use std::sync::Arc;
use std::thread;

struct Loader {
    command: String,
}

impl Loader {
    fn new() -> Self {
        // Insecure: a relative executable name is used which allows DLL preload or executable hijack in untrusted search paths.
        Loader {
            command: "example_app".to_string(),
        }
    }

    fn run(&self) -> Result<String, String> {
        // Unsafe block to mimic complex unsafe operations.
        unsafe {
            // Vulnerability: using a relative path in Command::new without validation.
            let output = Command::new(&self.command)
                .arg("--version")
                .output()
                .map_err(|e| format!("Failed execution: {}", e))?;
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        }
    }

    fn is_secure(&self) -> bool {
        // Check if the command path appears absolute.
        // This returns false if the command is relative.
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