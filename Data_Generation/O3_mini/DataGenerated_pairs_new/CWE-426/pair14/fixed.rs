//////////////////////////////////////////////////////////////
// Fixed Version - Secure executable loader enforcing absolute paths
//////////////////////////////////////////////////////////////

use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

struct Loader {
    // The path to the executable; must be an absolute path.
    path: String,
}

impl Loader {
    // Executes the external command only if the provided path is absolute.
    fn invoke(&self) -> Result<String, &'static str> {
        // Validate that the path is absolute.
        if !self.path.starts_with("/") {
            return Err("Non-absolute paths are not allowed");
        }

        // Execute the external command in a safe manner.
        let output = Command::new(self.path.clone())
            .output()
            .map_err(|_| "Command execution failed")?;
        String::from_utf8(output.stdout).map_err(|_| "Output was not valid UTF-8")
    }
}

fn main() {
    // Use an absolute path so that the loader does not reject the command.
    // In a real scenario, this would point to a trusted location.
    let loader = Arc::new(Mutex::new(Loader { path: "/usr/bin/trusted_bin".to_string() }));
    let loader_clone = loader.clone();
    let handle = thread::spawn(move || {
        let instance = loader_clone.lock().unwrap();
        match instance.invoke() {
            Ok(res) => println!("Result: {}", res),
            Err(err) => eprintln!("Error: {}", err),
        }
    });
    handle.join().unwrap();
}