use std::process::Command;
use std::thread;

struct Worker;

impl Worker {
    // This helper function returns the absolute file path of the executable.
    fn get_target(&self) -> &'static str {
        // Fix: The absolute path minimizes the risk by explicitly defining the executable location.
        "/usr/local/bin/helper_program"
    }

    fn run(&self) -> Result<String, String> {
        let target = self.get_target();
        // Directly execute the command with the secure, absolute path (no unsafe block needed).
        let output = Command::new(target)
            .output()
            .map_err(|e| e.to_string())?;
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }
}

fn main() {
    let worker = Worker;
    // Spawn a thread to simulate concurrent execution.
    let handle = thread::spawn(move || {
        match worker.run() {
            Ok(result) => println!("Output: {}", result),
            Err(e) => eprintln!("Error: {}", e),
        }
    });
    handle.join().unwrap();
}