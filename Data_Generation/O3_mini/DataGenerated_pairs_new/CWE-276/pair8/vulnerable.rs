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
        // Overly permissive mode: the file is created with mode 0o777.
        // This is vulnerable because it allows any user read/write/execute access.
        let file = unsafe {
            // The use of an unsafe block here is to simulate real-world codebases that wrap
            // lower-level OS interactions. However, the file permissions are set incorrectly.
            let options = OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .mode(0o777) // Vulnerability: incorrect default permissions.
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
    println!("Service executed with insecure file permissions");
}