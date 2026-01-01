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
        let offset = calc_offset(idx)?;
        let data_clone = Arc::clone(&self.data);
        let handle = thread::spawn(move || {
            unsafe {
                let lock = data_clone.lock().unwrap();
                let ptr = lock.as_ptr() as *mut u32;
                drop(lock);
                *ptr.add(offset) = val;
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
    if let Err(e) = mgr.update(5, 100) {
        println!("Error updating index 5: {}", e);
    }
    if let Err(e) = mgr.update(10, 200) {
        println!("Properly rejected update at index 10: {}", e);
    }
    if let Some(val) = mgr.get(5) {
        println!("Buffer[5] = {}", val);
    }
    if let Some(val) = mgr.get(10) {
        println!("Buffer[10] = {}", val);
    } else {
        println!("Buffer[10] is inaccessible");
    }
}
