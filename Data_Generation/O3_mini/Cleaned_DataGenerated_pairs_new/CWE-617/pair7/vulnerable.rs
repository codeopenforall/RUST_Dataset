use std::sync::Arc;
use std::thread;
use std::time::Duration;
pub struct DataHolder {
    pub data: Vec<u32>,
}
impl DataHolder {
    pub fn new(size: usize) -> Self {
        DataHolder { data: vec![0; size] }
    }
    pub unsafe fn get_ptr(&self) -> *const u32 {
        self.data.as_ptr()
    }
    pub fn process_input(&self, idx: usize) {
        unsafe {
            let ptr = self.get_ptr();
            let value = *ptr.add(idx);
            assert!(value != 42, "Invariant violated: unexpected trigger value encountered.");
        }
    }
}
pub fn critical_run() {
    let holder = Arc::new(DataHolder::new(10));
    let holder_clone = Arc::clone(&holder);
    let modify_handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        unsafe {
            let ptr = holder_clone.data.as_ptr() as *mut u32;
            *ptr.add(5) = 42;
        }
    });
    thread::sleep(Duration::from_millis(100));
    holder.process_input(5);
    modify_handle.join().unwrap();
}
fn main() {
    critical_run();
}
