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
        unsafe {
            let ptr = self.buffer.as_mut_ptr();
            debug_assert!(idx < len, "Index out-of-bounds in debug mode");
            assert!(idx < len, "Index out-of-bounds by attacker input");
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
        let _ = m.process(15);
    });
    let _ = t.join().unwrap();
    println!("Finished processing in vulnerable version");
}
