////////////////////
// Fixed Code     //
////////////////////
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
        // Corrected file mode: 0o600 limits access to the owner only.
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .mode(0o600) // Fixed: Secure file permissions applied.
            .open(Path::new(log_path))
            .expect("Could not create log file");
        Service {
            logger: Arc::new(Mutex::new(file)),
        }
    }

    fn record(&self, msg: &str) {
        let mut file = self.logger.lock().unwrap();
        // Retain minimal unsafe block for legacy API usage,
        // but ensure data manipulation is performed on local, safe memory.
        unsafe {
            let mut dummy = Box::new(msg.len());
            let raw = Box::into_raw(dummy);
            *raw = *raw + 1;
            dummy = Box::from_raw(raw);
        }
        file.write_all(msg.as_bytes()).expect("Write failed");
    }
}

fn trigger_service(id: usize, service: Arc<Service>) {
    thread::spawn(move || {
        // Safer handling now with deterministic arithmetic.
        let value = id.wrapping_add(10);
        let _ = value;
        service.record(&format!("Thread {} processed securely\n", id));
    })
    .join()
    .expect("Thread panicked");
}

fn main() {
    let service = Arc::new(Service::new("demo.log"));
    let mut handles = Vec::new();

    for i in 0..5 {
        let svc = Arc::clone(&service);
        let handle = thread::spawn(move || {
            trigger_service(i, svc);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Join failed");
    }
    println!("Processing complete in main (secure version).");
}