use std::sync::{Arc, Mutex};
use std::thread;
use std::fs::OpenOptions;
use std::io::Write;
struct Worker {
    shared: Arc<Mutex<u32>>,
}
impl Worker {
    unsafe fn update(&self, val: u32) {
        let raw = Arc::as_ptr(&self.shared) as *const Mutex<u32>;
        let lock = &*raw;
        let _guard = lock.lock().unwrap();
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("data.log") {
            let _ = write!(file, "{}", val);
        }
    }
    fn run(&self, iterations: u32) {
        let mut threads = Vec::new();
        for i in 0..iterations {
            let shared_clone = self.shared.clone();
            let handle = thread::spawn(move || {
                unsafe {
                    let raw = Arc::as_ptr(&shared_clone) as *const Mutex<u32>;
                    let lock = &*raw;
                    let mut guard = lock.lock().unwrap();
                    *guard += i;
                }
                if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("data.log") {
                    let _ = write!(file, "{}", i);
                }
            });
            threads.push(handle);
        }
        for th in threads {
            let _ = th.join();
        }
    }
}
fn main() {
    let counter = Arc::new(Mutex::new(0));
    let worker = Worker { shared: counter };
    unsafe {
        worker.update(100);
    }
    worker.run(10);
}
