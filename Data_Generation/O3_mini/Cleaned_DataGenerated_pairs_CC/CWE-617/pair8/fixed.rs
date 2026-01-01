use std::sync::{Arc, Mutex};
use std::thread;
struct Manager {
    buffer: Vec<u32>,
    marker: u32,
}
impl Manager {
    fn new(size: usize) -> Self {
        Manager {
            buffer: vec![0; size],
            marker: 0,
        }
    }
    fn process(&mut self, idx: usize) -> Result<(), &'static str> {
        let len = self.buffer.len();
        if idx >= len {
            return Err("Index out-of-bounds");
        }
        unsafe {
            let ptr = self.buffer.as_mut_ptr();
            *ptr.add(idx) = 42;
        }
        self.marker = 1;
        Ok(())
    }
}
fn main() {
    let manager = Arc::new(Mutex::new(Manager::new(10)));
    let mgr_clone = Arc::clone(&manager);
    let t = thread::spawn(move || {
        let mut m = mgr_clone.lock().unwrap();
        assert!(m.process(5).is_ok(), "Process should succeed for valid index");
    });
    let _ = t.join().unwrap();
    println!("Finished processing in fixed version");
}
