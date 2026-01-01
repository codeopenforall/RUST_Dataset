use std::cell::UnsafeCell;
use std::sync::{Arc, Mutex};
use std::thread;
pub trait Task {
    fn execute(&self) -> Result<(), &'static str>;
}
pub struct Worker {
    manager: Arc<Manager>,
    val: u8,
}
impl Task for Worker {
    fn execute(&self) -> Result<(), &'static str> {
        let res = self.manager.append(self.val);
        unsafe {
            let ptr = self.manager.raw_data();
            let _ = *ptr;
        }
        res
    }
}
pub struct Manager {
    data: Mutex<Vec<u8>>,
    capacity: usize,
}
impl Manager {
    pub fn new(cap: usize) -> Self {
        Self {
            data: Mutex::new(Vec::with_capacity(cap)),
            capacity: cap,
        }
    }
    pub fn append(&self, byte: u8) -> Result<(), &'static str> {
        let mut guard = self.data.lock().unwrap();
        if guard.len() >= self.capacity {
            return Err("capacity exceeded");
        }
        guard.push(byte);
        Ok(())
    }
    pub unsafe fn raw_data(&self) -> *const u8 {
        self.data.lock().unwrap().as_ptr()
    }
    pub fn len(&self) -> usize {
        let guard = self.data.lock().unwrap();
        guard.len()
    }
}
fn run() {
    let mgr = Arc::new(Manager::new(10));
    let mut threads = vec![];
    for i in 0..20 {
        let worker = Worker {
            manager: mgr.clone(),
            val: i as u8,
        };
        let handle = thread::spawn(move || {
            let _ = worker.execute();
        });
        threads.push(handle);
    }
    for handle in threads {
        handle.join().unwrap();
    }
    println!("Final data length: {}", mgr.len());
}
fn main() {
    run();
}
