////////////////////////////////////////////////////////////////////////////////////////////////////
// This Rust program implements a log server that writes to a file concurrently using threads.
// It mistakenly creates/open the log file with overly permissive access rights (0o777) by using
// an unsafe block. This relaxation of permissions exposes the file to unauthorized read/write,
// thereby causing CWE-276 (Incorrect Default Permissions) vulnerabilities.
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
        // Using an unsafe block to bypass normal checks (simulated).
        // The file is created with mode 0o777. This is overly permissive.
        let file = unsafe {
            OpenOptions::new()
                .write(true)
                .create(true)
                .mode(0o777)  // Vulnerability: insecure default permissions
                .open(path)?
        };
        let log_file = Arc::new(Mutex::new(file));
        Ok(LogServer { log_file })
    }

    fn run(&self) {
        let log_file = Arc::clone(&self.log_file);
        // Spawn a thread to simulate concurrent logging
        let handle = thread::spawn(move || {
            let mut file = log_file.lock().unwrap();
            file.write_all(b"Insecure log entry\n").unwrap();
        });
        handle.join().unwrap();
    }
}

fn main() {
    let server = LogServer::new("server.log").expect("Failed to create log server");
    server.run();
}