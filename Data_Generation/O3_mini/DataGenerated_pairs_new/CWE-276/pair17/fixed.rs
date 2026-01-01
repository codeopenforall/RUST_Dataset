////////////////////////////////////////////////////////////////////////////////////////////////////
// This Rust program implements a log server in a secure way. It creates and writes to a file 
// concurrently using threads and smart pointers. The file is now created with restrictive permissions (0o600),
// ensuring that only the file owner has read and write access. This fixes the CWE-276 vulnerability.
////////////////////////////////////////////////////////////////////////////////////////////////////

use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::sync::{Arc, Mutex};
use std::thread;
use std::io::Write;

struct LogServer {
    log_file: Arc<Mutex<std::fs::File>>,
}

impl LogServer {
    fn new(path: &str) -> std::io::Result<Self> {
        // Securely create/open the file with restrictive permissions (0o600)
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .mode(0o600)  // Secure permissions: only owner allowed
            .open(path)?;
        let log_file = Arc::new(Mutex::new(file));
        Ok(LogServer { log_file })
    }

    fn run(&self) {
        let log_file = Arc::clone(&self.log_file);
        // Spawn a thread for concurrent logging
        let handle = thread::spawn(move || {
            let mut file = log_file.lock().unwrap();
            file.write_all(b"Secure log entry\n").unwrap();
        });
        handle.join().unwrap();
    }
}

fn main() {
    let server = LogServer::new("server.log").expect("Failed to create log server");
    server.run();
}