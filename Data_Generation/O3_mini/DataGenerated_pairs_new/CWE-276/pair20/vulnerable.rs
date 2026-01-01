/*
   insecure_demo.rs
   A simulation of a server that creates a file with overly permissive file permissions,
   potentially leading to unauthorized access. The file is created using an unsafe block
   and the Unix-specific OpenOptionsExt with a mode of 0o777.
*/

use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Server {
    path: String,
    state: Arc<Mutex<u32>>,
}

impl Server {
    // Line 14
    fn new(path: String) -> Self {
        Server {
            path,
            state: Arc::new(Mutex::new(0)),
        }
    }

    // Line 21: Entry point for the simulated operation.
    fn run(&self) -> std::io::Result<()> {
        let file_path = self.path.clone();
        let state_clone = Arc::clone(&self.state);
        let handle = thread::spawn(move || {
            // Unsafe block used to simulate low-level file manipulation.
            let file = unsafe {
                // Vulnerability: File is created with overly permissive mode 0o777.
                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .mode(0o777) // Vulnerable line (line 29)
                    .open(&file_path)
            };
            file.expect("File creation failed");

            // Simulate some state update
            {
                let mut data = state_clone.lock().unwrap();
                *data += 1;
            }

            // Simulate processing delay.
            thread::sleep(Duration::from_millis(50));
        });
        handle.join().expect("Thread panicked");
        Ok(())
    }
}

fn main() {
    // Line 43: Main entry point; creates an instance and runs it.
    let srv = Server::new("demo_insecure.txt".to_string());
    if let Err(e) = srv.run() {
        eprintln!("Error: {}", e);
    } else {
        println!("Operation completed.");
    }
}