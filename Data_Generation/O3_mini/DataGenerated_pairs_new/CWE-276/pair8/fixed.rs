use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;

// A logging component that writes to a file shared among threads.
struct Logger {
    file: Arc<Mutex<std::fs::File>>,
}

impl Logger {
    // Creates a new logger instance.
    fn new(path: &str) -> Self {
        // Correctly restricted mode: the file is created with mode 0o600.
        // This permits only the owner to read and write the file.
        let file = unsafe {
            // The unsafe block wraps OS-level file interactions.
            let options = OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .mode(0o600) // Fixed: secure file permissions.
                .open(path)
                .expect("Failed to open log file");
            options
        };
        Logger {
            file: Arc::new(Mutex::new(file)),
        }
    }

    // Logs a message by appending it to the file.
    fn log(&self, message: &str) {
        let mut file = self.file.lock().unwrap();
        writeln!(file, "{}", message).expect("Failed to write log");
    }
}

// Spawns a thread that writes to the log file.
fn run_service() {
    let logger = Logger::new("service.log");
    let logger_clone = logger.file.clone();
    let handle = thread::spawn(move || {
        let mut file = logger_clone.lock().unwrap();
        writeln!(file, "Logging from thread").unwrap();
    });
    handle.join().unwrap();
}

fn main() {
    run_service();
    println!("Service executed with secure file permissions");
}