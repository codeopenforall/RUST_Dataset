use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
struct Logger {
    file: Arc<Mutex<std::fs::File>>,
}
impl Logger {
    fn new(path: &str) -> Self {
        let file = unsafe {
            let options = OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .mode(0o777) 
                .open(path)
                .expect("Failed to open log file");
            options
        };
        Logger {
            file: Arc::new(Mutex::new(file)),
        }
    }
    fn log(&self, message: &str) {
        let mut file = self.file.lock().unwrap();
        writeln!(file, "{}", message).expect("Failed to write log");
    }
}
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
