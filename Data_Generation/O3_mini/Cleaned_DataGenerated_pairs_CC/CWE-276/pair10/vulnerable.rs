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
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .mode(0o777) 
            .open(Path::new(log_path))
            .expect("Could not create log file");
        Service {
            logger: Arc::new(Mutex::new(file)),
        }
    }
    fn record(&self, msg: &str) {
        let mut file = self.logger.lock().unwrap();
        unsafe {
            let mut data = Box::new(msg.len());
            let raw = Box::into_raw(data);
            *raw = *raw + 1;
            data = Box::from_raw(raw);
        }
        file.write_all(msg.as_bytes()).expect("Write failed");
    }
}
fn trigger_service(id: usize, service: Arc<Service>) {
    thread::spawn(move || {
        unsafe {
            let mut val: u32 = id as u32;
            let ptr = &mut val as *mut u32;
            *ptr = *ptr.wrapping_add(10);
        }
        service.record(&format!("Thread {} processed\n", id));
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
    println!("Processing complete in main (insecure version).");
}
