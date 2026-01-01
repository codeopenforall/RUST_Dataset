/*
   secure_demo.rs
   A simulation of a server that creates a file with appropriate, minimal file permissions.
   The file is now created with a restrictive mode (0o600) using safe practices with the 
   Unix-specific OpenOptionsExt. An unsafe block is still present for compatibility reasons,
   but it does not introduce a vulnerability.
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
    fn new(path: String) -> Self {
        Server {
            path,
            state: Arc::new(Mutex::new(0)),
        }
    }

    fn run(&self) -> std::io::Result<()> {
        let file_path = self.path.clone();
        let state_clone = Arc::clone(&self.state);
        let handle = thread::spawn(move || {
            // Use an unsafe block but now with secure file mode settings.
            let file = unsafe {
                // Fix: Use mode 0o600 to restrict file permissions to the owner.
                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .mode(0o600) // Secure file creation (line 29)
                    .open(&file_path)
            };
            file.expect("File creation failed");

            // Update internal state safely.
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
    let srv = Server::new("demo_secure.txt".to_string());
    if let Err(e) = srv.run() {
        eprintln!("Error: {}", e);
    } else {
        println!("Operation completed.");
    }
}