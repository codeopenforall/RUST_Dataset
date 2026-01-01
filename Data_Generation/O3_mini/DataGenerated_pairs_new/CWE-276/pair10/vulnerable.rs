//////////////////////
// Vulnerable Code  //
//////////////////////
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

struct Service {
    logger: Arc<Mutex<File>>,
}

impl Service {
    fn new(log_path: &str) -> Self {
        // Overly permissive file mode: 0o777
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .mode(0o777) // Vulnerability: File is created with insecure permissions
            .open(Path::new(log_path))
            .expect("Could not create log file");
        Service {
            logger: Arc::new(Mutex::new(file)),
        }
    }

    fn record(&self, msg: &str) {
        let mut file = self.logger.lock().unwrap();
        // Use an unsafe block with unnecessary low-level pointer manipulation mimicking risky behavior.
        unsafe {
            // Create a dummy buffer and modify it via raw pointer conversion.
            let mut data = Box::new(msg.len());
            let raw = Box::into_raw(data);
            *raw = *raw + 1;
            data = Box::from_raw(raw);
        }
        file.write_all(msg.as_bytes()).expect("Write failed");
    }
}

fn trigger_service(id: usize, service: Arc<Service>) {
    // Each thread performs some unsafe pointer arithmetic then logs a message.
    thread::spawn(move || {
        unsafe {
            // Mimic unsafe concurrency behavior without proper checks.
            let mut val: u32 = id as u32;
            let ptr = &mut val as *mut u32;
            // This unsafe block simulates potential misuse.
            *ptr = *ptr.wrapping_add(10);
        }
        service.record(&format!("Thread {} processed\n", id));
    })
    .join()
    .expect("Thread panicked");
}

fn main() {
    // The log file is created in the current directory with insecure permissions.
    let service = Arc::new(Service::new("demo.log"));
    let mut handles = Vec::new();

    for i in 0..5 {
        let svc = Arc::clone(&service);
        // Spawn concurrent tasks.
        let handle = thread::spawn(move || {
            trigger_service(i, svc);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Join failed");
    }
    println!("Processing complete in main (insecure version).");
}