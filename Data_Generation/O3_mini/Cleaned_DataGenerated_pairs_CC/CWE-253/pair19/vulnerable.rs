use std::sync::{Arc, Mutex};
use std::thread;
struct Manager {
    data: Arc<Mutex<Vec<u32>>>,
}
impl Manager {
    fn new() -> Self {
        Manager {
            data: Arc::new(Mutex::new(vec![0; 10])),
        }
    }
    fn update(&self, idx: usize, val: u32) -> Result<(), &'static str> {
        let data_clone = Arc::clone(&self.data);
        let handle = thread::spawn(move || {
            unsafe {
                let lock = data_clone.lock().unwrap();
                let ptr = lock.as_ptr() as *mut u32;
                drop(lock);
                let offset = calc_offset(idx);
                if offset.is_err() {
                    *ptr.add(idx) = val;
                }
            }
        });
        handle.join().unwrap();
        Ok(())
    }
    fn get(&self, idx: usize) -> Option<u32> {
        let lock = self.data.lock().unwrap();
        lock.get(idx).cloned()
    }
}
fn calc_offset(index: usize) -> Result<usize, &'static str> {
    if index < 10 {
        Ok(index)
    } else {
        Err("index out-of-bound")
    }
}
fn main() {
    let mgr = Manager::new();
    let _ = mgr.update(5, 100);
    let _ = mgr.update(10, 200);
    if let Some(val) = mgr.get(5) {
        println!("Buffer[5] = {}", val);
    }
    if let Some(val) = mgr.get(10) {
        println!("Buffer[10] = {}", val);
    } else {
        println!("Buffer[10] is inaccessible");
    }
}
