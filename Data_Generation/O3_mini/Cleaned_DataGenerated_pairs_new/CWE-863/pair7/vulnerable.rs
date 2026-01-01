use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
struct Worker {
    id: usize,
    data: Vec<u8>,
}
impl Worker {
    unsafe fn process(&mut self) {
        let len = self.data.len();
        let ptr = self.data.as_mut_ptr();
        for i in 0..len {
            *ptr.add(i) = (i % 256) as u8;
        }
    }
}
fn run_tasks(task_count: usize) {
    let shared = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    for i in 0..task_count {
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            let mut worker = Worker {
                id: i,
                data: Vec::with_capacity(1024 * 1024), 
            };
            unsafe {
                worker.data.set_len(1024 * 1024);
                worker.process();
            }
            let mut guard = shared_clone.lock().unwrap();
            guard.push(worker.id);
        });
        handles.push(handle);
    }
    for h in handles {
        let _ = h.join();
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let count: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(0)
    } else {
        0
    };
    run_tasks(count);
    println!("Completed tasks");
}
